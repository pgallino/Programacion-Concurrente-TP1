//! Este módulo contiene funciones y tipos de datos para el procesamiento de archivos JSON.

use crate::structs::{
    JsonStructure, LineJsonStructure, ResultData, SiteData, TagData, CHATTY_TAGS_MAX,
};
use rayon::prelude::*;
use serde_json::from_str;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process;

/// Cantidad de preguntas por línea.
const QUESTIONS_PER_LINE: u32 = 1;

/// Máximo número de sitios "chatty".
const CHATTY_SITES_MAX: usize = 10;

/// Número de padrón.
const PADRON: u32 = 107587;

/// Cuenta la cantidad de palabras en un vector de cadenas de texto.
///
/// # Arguments
///
/// * `texts` - Vector de cadenas de texto.
///
/// # Returns
///
/// La cantidad total de palabras en el vector de cadenas de texto.
pub fn word_counter(texts: &[String]) -> u32 {
    texts
        .iter()
        .map(|text| text.split_whitespace().count() as u32)
        .sum()
}

/// Lista los archivos en un directorio.
///
/// # Arguments
///
/// * `directory` - Ruta del directorio.
///
/// # Returns
///
/// Un vector de rutas de archivos dentro del directorio especificado.
///
/// # Errors
///
/// Puede devolver un error si no puede leer el directorio especificado.
pub fn list_files(directory: &str) -> Vec<PathBuf> {
    let dir_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), directory);
    match read_dir(&dir_path) {
        Ok(entries) => entries.flatten().map(|entry| entry.path()).collect(),
        Err(e) => {
            eprintln!("Error al leer el directorio {}: {}", dir_path, e);
            process::exit(1);
        }
    }
}

/// Toma una estructura `LineJsonStructure` y genera un sub `ResultData` con su información.
///
/// # Arguments
///
/// * `line_struct` - Estructura `LineJsonStructure` que contiene la información de una línea.
/// * `json_struct` - Estructura `JsonStructure` que contiene la información del archivo JSON.
///
/// # Returns
///
/// Un `ResultData` generado a partir de la línea y la estructura JSON dadas.
pub fn generate_result_data_from_line(
    line_struct: LineJsonStructure,
    mut json_struct: JsonStructure,
) -> ResultData {
    json_struct.load_info(line_struct);

    let words_count = word_counter(&json_struct.texts);
    let mut tags: HashMap<String, TagData> = HashMap::new();
    for tag in &json_struct.tags {
        let tag_data = TagData::new(QUESTIONS_PER_LINE, words_count);
        tags.insert(tag.clone(), tag_data);
    }
    let site_data = SiteData::new(QUESTIONS_PER_LINE, words_count, tags);
    let mut site_subhash: HashMap<String, SiteData> = HashMap::new();
    site_subhash.insert(json_struct.site.clone(), site_data.clone());
    ResultData::new(PADRON, site_subhash, site_data.tags)
}

/// Obtiene el nombre del sitio del archivo.
///
/// # Arguments
///
/// * `path` - Ruta del archivo.
///
/// # Returns
///
/// El nombre del sitio extraído del nombre del archivo.
fn get_site_name(path: &Path) -> String {
    path.file_name()
        .map(|p| p.to_string_lossy().trim_end_matches(".jsonl").to_string())
        .unwrap_or_else(|| {
            eprintln!("Error al obtener el nombre del archivo.");
            process::exit(1);
        })
}

/// Procesa las líneas del archivo y genera un `ResultData`.
///
/// # Arguments
///
/// * `reader` - `BufReader` para leer el archivo.
/// * `site_name` - Nombre del sitio del archivo.
///
/// # Returns
///
/// Un `ResultData` generado a partir del procesamiento de las líneas del archivo.
fn process_lines(reader: BufReader<File>, site_name: &str) -> ResultData {
    reader
        .lines()
        .par_bridge()
        .filter_map(|line_result| {
            let line = match line_result {
                Ok(line) => line,
                Err(e) => {
                    eprintln!("Error al leer línea del archivo: {}", e);
                    return None;
                }
            };

            match from_str::<LineJsonStructure>(&line) {
                Ok(data) => Some(generate_result_data_from_line(
                    data,
                    JsonStructure::new(site_name.to_string()),
                )),
                Err(e) => {
                    eprintln!("Error al analizar JSON en línea del archivo: {}", e);
                    Some(ResultData::new(PADRON, HashMap::new(), HashMap::new()))
                }
            }
        })
        .reduce(
            || ResultData::new(PADRON, HashMap::new(), HashMap::new()),
            |acc, b| acc + b,
        )
}

/// Procesa un archivo individual y genera un `ResultData`.
///
/// # Arguments
///
/// * `path` - Ruta del archivo a procesar.
///
/// # Returns
///
/// Un `ResultData` generado a partir del procesamiento del archivo.
pub fn process_file(path: &PathBuf) -> ResultData {
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let site_name = get_site_name(path);

            process_lines(reader, &site_name)
        }
        Err(e) => {
            eprintln!("Error al abrir archivo {}: {}", path.display(), e);
            process::exit(1);
        }
    }
}

/// Procesa una lista de archivos y devuelve un `ResultData` combinado.
///
/// # Arguments
///
/// * `paths` - Vector de rutas de archivos a procesar.
///
/// # Returns
///
/// Un `ResultData` combinado a partir del procesamiento de los archivos.
pub fn process_files(paths: &[PathBuf]) -> ResultData {
    let results_per_file: Vec<ResultData> = paths.par_iter().map(process_file).collect();

    let mut combined_result = ResultData::new(PADRON, HashMap::new(), HashMap::new());
    for result in results_per_file {
        combined_result = combined_result + result;
    }
    combined_result
}

/// Calcula el campo Totals de un `ResultData`.
///
/// # Arguments
///
/// * `result_data` - Referencia mutable a un `ResultData` que se va a procesar.
pub fn process_totals(result_data: &mut ResultData) {
    result_data.totals.chatty_sites = process_sites(&mut result_data.sites);
    result_data.totals.chatty_tags = process_tags(&result_data.tags);
}

/// Ordena un vector de tuplas por coeficiente y nombre.
///
/// # Arguments
///
/// * `data` - Vector de tuplas a ordenar.
fn sort_by_coef_and_name<T>(data: &mut [(String, T)])
where
    T: PartialOrd,
{
    data.sort_by(|a, b| {
        let coef_cmp = b.1.partial_cmp(&a.1).unwrap();
        if coef_cmp != std::cmp::Ordering::Equal {
            return coef_cmp;
        }
        a.0.partial_cmp(&b.0).unwrap()
    });
}

/// Procesa los sitios y devuelve una lista de los más "chatty".
///
/// # Arguments
///
/// * `sites_data` - Referencia mutable a un mapa de datos de sitios.
///
/// # Returns
///
/// Una lista de nombres de sitios que son los más "chatty".
pub fn process_sites(sites_data: &mut HashMap<String, SiteData>) -> Vec<String> {
    let mut top_sites: Vec<_> = sites_data
        .par_iter_mut()
        .map(|(site, data)| {
            data.load_chatty_tags(process_tags(&data.tags));
            (site.clone(), data.get_coef())
        })
        .collect();

    sort_by_coef_and_name(&mut top_sites);

    top_sites
        .into_iter()
        .take(CHATTY_SITES_MAX)
        .map(|(site, _)| site)
        .collect()
}

/// Procesa las etiquetas y devuelve una lista de las más "chatty".
///
/// # Arguments
///
/// * `tags_data` - Referencia a un mapa de datos de etiquetas.
///
/// # Returns
///
/// Una lista de nombres de etiquetas que son las más "chatty".
pub fn process_tags(tags_data: &HashMap<String, TagData>) -> Vec<String> {
    let mut top_tags: Vec<_> = tags_data
        .par_iter()
        .map(|(tag, data)| (tag.clone(), data.get_coef()))
        .collect();

    sort_by_coef_and_name(&mut top_tags);

    top_tags
        .into_iter()
        .take(CHATTY_TAGS_MAX)
        .map(|(tag, _)| tag)
        .collect()
}
