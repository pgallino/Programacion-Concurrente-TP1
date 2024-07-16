use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Contiene el máximo número de etiquetas "chatty".
pub(crate) const CHATTY_TAGS_MAX: usize = 10;

/// LineJsonStructure: Contiene la información de una linea del json.

#[derive(Debug, Deserialize)]
pub(crate) struct LineJsonStructure {
    pub(crate) texts: Vec<String>,
    pub(crate) tags: Vec<String>,
}

/// JsonStructure: Envuelve LineJsonStructure y le agrega el nombre del sitio.

#[derive(Debug, Deserialize)]
pub(crate) struct JsonStructure {
    pub(crate) site: String,
    pub(crate) texts: Vec<String>,
    pub(crate) tags: Vec<String>,
}

impl JsonStructure {
    /// Crea una nueva instancia de `JsonStructure`.
    ///
    /// # Arguments
    ///
    /// * `site` - Nombre del sitio a asociar con la estructura JSON.
    ///
    /// # Returns
    ///
    /// Una nueva instancia de `JsonStructure` con el nombre del sitio especificado.
    pub(crate) fn new(site: String) -> Self {
        JsonStructure {
            site,
            texts: vec![],
            tags: vec![],
        }
    }

    /// Carga la información de un `LineJsonStructure` en la estructura actual.
    ///
    /// # Arguments
    ///
    /// * `other` - Estructura `LineJsonStructure` que contiene la información a cargar.
    pub(crate) fn load_info(&mut self, other: LineJsonStructure) {
        // Sumar los valores de questions y words del otro TagData al actual
        self.texts = other.texts;
        self.tags = other.tags;
    }
}

/// ResultData: Contiene la información TOTAL. Se utiliza para expresar el resultado final y pasarlo a json.

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ResultData {
    pub(crate) padron: u32,
    pub(crate) sites: HashMap<String, SiteData>,
    pub(crate) tags: HashMap<String, TagData>,
    pub(crate) totals: TotalsData,
}

impl ResultData {
    /// Crea una nueva instancia de `ResultData`.
    ///
    /// # Arguments
    ///
    /// * `padron` - Número de padron.
    /// * `sites` - Map de sitios con sus datos asociados.
    /// * `tags` - Map de etiquetas con sus datos asociados.
    ///
    /// # Returns
    ///
    /// Una nueva instancia de `ResultData` con los datos proporcionados.
    pub(crate) fn new(
        padron: u32,
        sites: HashMap<String, SiteData>,
        tags: HashMap<String, TagData>,
    ) -> ResultData {
        ResultData {
            padron,
            sites,
            tags,
            totals: TotalsData {
                chatty_sites: vec![],
                chatty_tags: vec![],
            },
        }
    }
}

/// Realiza la suma entre dos instancias de `ResultData`.
///
/// # Arguments
///
/// * `self` - La primera instancia de `ResultData`.
/// * `other` - La segunda instancia de `ResultData` que se suma a la primera.
///
/// # Returns
///
/// Un `ResultData` con la suma de ambas instancias.
impl std::ops::Add for ResultData {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        reduce(&mut self.sites, other.sites);
        reduce(&mut self.tags, other.tags);

        ResultData {
            padron: self.padron,
            sites: self.sites,
            tags: self.tags,
            totals: self.totals,
        }
    }
}

/// SiteData: Contiene la información de un SITE

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(crate) struct SiteData {
    pub(crate) questions: u32,
    pub(crate) words: u32,
    pub(crate) tags: HashMap<String, TagData>,
    pub(crate) chatty_tags: Vec<String>,
}

impl SiteData {
    /// Crea una nueva instancia de `SiteData`.
    ///
    /// # Arguments
    ///
    /// * `questions` - Número de preguntas.
    /// * `words` - Número de palabras.
    /// * `tags` - Map de etiquetas con sus datos asociados.
    ///
    /// # Returns
    ///
    /// Una nueva instancia de `SiteData` con los datos proporcionados.
    pub(crate) fn new(questions: u32, words: u32, tags: HashMap<String, TagData>) -> Self {
        SiteData {
            questions,
            words,
            tags,
            chatty_tags: vec![String::new(); CHATTY_TAGS_MAX],
        }
    }
    /// Calcula y devuelve el coeficiente chatty para el sitio.
    ///
    /// # Returns
    ///
    /// El coeficiente de chatty para el sitio.
    pub(crate) fn get_coef(&self) -> u32 {
        self.words / self.questions
    }

    /// Carga las etiquetas "chatty" en el sitio.
    ///
    /// # Arguments
    ///
    /// * `chatty_tags` - Etiquetas "chatty" a cargar en el sitio.
    pub(crate) fn load_chatty_tags(&mut self, chatty_tags: Vec<String>) {
        self.chatty_tags = chatty_tags;
    }
}

/// TagData: Contiene la información de un Tag
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct TagData {
    pub(crate) questions: u32,
    pub(crate) words: u32,
}

impl TagData {
    /// Crea una nueva instancia de `TagData`.
    ///
    /// # Arguments
    ///
    /// * `questions` - Número de preguntas.
    /// * `words` - Número de palabras.
    ///
    /// # Returns
    ///
    /// Una nueva instancia de `TagData` con los datos proporcionados.
    pub(crate) fn new(questions: u32, words: u32) -> Self {
        TagData { questions, words }
    }

    /// Calcula y devuelve el coeficiente chatty para la etiqueta.
    ///
    /// # Returns
    ///
    /// El coeficiente de chatty para la etiqueta.
    pub(crate) fn get_coef(&self) -> u32 {
        self.words / self.questions
    }
}

/// TotalsData: Contiene la información total de los totales.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TotalsData {
    pub(crate) chatty_sites: Vec<String>,
    pub(crate) chatty_tags: Vec<String>,
}

/// Trait que permite la reducción de las estructuras `SiteData` y `TagData`.
trait Reducible {
    /// Combina dos instancias de la estructura.
    ///
    /// # Arguments
    ///
    /// * `other` - Otra instancia de la estructura a combinar con la actual.
    fn combine(&mut self, other: Self);
}

/// Implementación del trait `Reducible` para `SiteData`.
impl Reducible for SiteData {
    fn combine(&mut self, other: Self) {
        // Implementa la lógica de combinación para SiteData
        self.questions += other.questions;
        self.words += other.words;
        reduce(&mut self.tags, other.tags);
    }
}

/// Implementación del trait `Reducible` para `TagData`.
impl Reducible for TagData {
    fn combine(&mut self, other: Self) {
        // Implementa la lógica de combinación para TagData
        self.questions += other.questions;
        self.words += other.words;
    }
}

/// Función genérica para reducir dos HashMaps
fn reduce<T: Reducible + Clone>(destino: &mut HashMap<String, T>, origen: HashMap<String, T>) {
    for (key, value) in origen {
        match destino.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(value.clone());
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().combine(value);
            }
        }
    }
}
