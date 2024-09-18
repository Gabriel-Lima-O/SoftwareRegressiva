use serde_derive::{Deserialize, Serialize};
use sdl2::pixels::Color;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

/// Estrutura principal de configuração
#[derive(Debug, Deserialize, Serialize)]
pub struct RegressivaConfig {
    #[serde(rename = "cores")]
   pub cores: CoresConfig,

    #[serde(rename = "tempo")]
    pub tempo: TempoConfig,

    #[serde(rename = "tempo_texto")]
    pub tempo_texto: TempoTextoConfig,

    #[serde(rename = "creditos")]
    pub  creditos: CreditosConfig,

    #[serde(rename = "botoes")]
    pub botoes: BotoesConfig,

    #[serde(rename = "debug")]
    pub debug: DebugConfig,

    #[serde(rename = "fontes")]
    pub fontes: FontesConfig,

    #[serde(rename = "boas_vindas")]
    pub boas_vindas: BoasVindasConfig,
}

impl Default for RegressivaConfig {
    fn default() -> Self {
        RegressivaConfig {
            cores: CoresConfig::default(),
            tempo: TempoConfig::default(),
            tempo_texto: TempoTextoConfig::default(),
            creditos: CreditosConfig::default(),
            botoes: BotoesConfig::default(),
            debug: DebugConfig::default(),
            fontes: FontesConfig::default(),
            boas_vindas: BoasVindasConfig::default(),
        }
    }
}

/// Configuração das cores
#[derive(Debug, Deserialize, Serialize)]
pub struct CoresConfig {
    cor_background: [u8; 3],
    cor_botoes: [u8; 3],
    cor_vermelho: [u8; 3],
    cor_verde: [u8; 3],
    cor_azul: [u8; 3],
    cor_texto: [u8; 3],
}

impl CoresConfig {
    /// Retorna a cor de fundo
    pub fn get_background_color(&self) -> Color {
        Color::RGB(
            self.cor_background[0],
            self.cor_background[1],
            self.cor_background[2],
        )
    }

    /// Retorna a cor dos botões
    pub fn get_button_color(&self) -> Color {
        Color::RGB(self.cor_botoes[0], self.cor_botoes[1], self.cor_botoes[2])
    }

    /// Retorna a cor vermelha com transparência
    pub fn get_red_color(&self) -> Color {
        Color::RGBA(
            self.cor_vermelho[0],
            self.cor_vermelho[1],
            self.cor_vermelho[2],
            150,
        )
    }

    /// Retorna a cor verde com transparência
    pub fn get_green_color(&self) -> Color {
        Color::RGBA(self.cor_verde[0], self.cor_verde[1], self.cor_verde[2], 150)
    }

    /// Retorna a cor azul com transparência
    pub fn get_blue_color(&self) -> Color {
        Color::RGBA(self.cor_azul[0], self.cor_azul[1], self.cor_azul[2], 150)
    }

    /// Retorna a cor do texto
    pub fn get_text_color(&self) -> Color {
        Color::RGB(self.cor_texto[0], self.cor_texto[1], self.cor_texto[2])
    }
}

impl Default for CoresConfig {
    fn default() -> Self {
        CoresConfig {
            cor_background: [9, 61, 83],
            cor_botoes: [255, 255, 255],
            cor_vermelho: [100, 0, 0],
            cor_verde: [0, 100, 0],
            cor_azul: [0, 0, 100],
            cor_texto: [0, 0, 0],
        }
    }
}

/// Configuração das boas-vindas
#[derive(Debug, Deserialize, Serialize)]
pub struct BoasVindasConfig {
    mostrar_boas_vindas: bool,
}

impl BoasVindasConfig {
    /// Retorna se deve mostrar as boas-vindas
    pub fn mostrar_boas_vindas(&self) -> bool {
        self.mostrar_boas_vindas
    }
}

impl Default for BoasVindasConfig {
    fn default() -> Self {
        BoasVindasConfig {
            mostrar_boas_vindas: true,
        }
    }
}

/// Configuração das fontes
#[derive(Debug, Deserialize, Serialize)]
pub struct FontesConfig {
    fonte: String,
    fonte_gigante: u16,
    fonte_media: u16,
    fonte_grande: u16,
    fonte_pequena: u16,
}

impl FontesConfig {
    /// Retorna o nome do arquivo de fonte
    pub fn get_fonte(&self) -> &str {
        &self.fonte
    }

    /// Retorna o tamanho da fonte gigante
    pub fn get_fonte_gigante(&self) -> u16 {
        self.fonte_gigante
    }

    /// Retorna o tamanho da fonte média
    pub fn get_fonte_media(&self) -> u16 {
        self.fonte_media
    }

    /// Retorna o tamanho da fonte grande
    pub fn get_fonte_grande(&self) -> u16 {
        self.fonte_grande
    }

    /// Retorna o tamanho da fonte pequena
    pub fn get_fonte_pequena(&self) -> u16 {
        self.fonte_pequena
    }
}

impl Default for FontesConfig {
    fn default() -> Self {
        FontesConfig {
            fonte: "GlobotipoVariable-VF.ttf".to_string(),
            fonte_gigante: 64,
            fonte_media: 32,
            fonte_grande: 24,
            fonte_pequena: 16,
        }
    }
}

/// Configuração dos tempos
#[derive(Debug, Deserialize, Serialize)]
pub struct TempoConfig {
    tempo1: u64,
    tempo2: u64,
    tempo3: u64,
    tempo4: u64,
    tempo5: u64,
    tempo6: u64,
    tempo7: u64,
    tempo8: u64,
    tempo9: u64,
    tempo_reset: u64,
}

impl TempoConfig {
    /// Retorna o tempo configurado para o índice especificado
    pub fn get_tempo(&self, index: usize) -> u64 {
        match index {
            1 => self.tempo1,
            2 => self.tempo2,
            3 => self.tempo3,
            4 => self.tempo4,
            5 => self.tempo5,
            6 => self.tempo6,
            7 => self.tempo7,
            8 => self.tempo8,
            9 => self.tempo9,
            _ => self.tempo_reset,
        }
    }

    /// Retorna o tempo de reset
    pub fn get_tempo_reset(&self) -> u64 {
        self.tempo_reset
    }
}

impl Default for TempoConfig {
    fn default() -> Self {
        TempoConfig {
            tempo1: 30,
            tempo2: 60,
            tempo3: 75,
            tempo4: 120,
            tempo5: 1200,
            tempo6: 0,
            tempo7: 120,
            tempo8: 1200,
            tempo9: 0,
            tempo_reset: 14,
        }
    }
}

/// Configuração dos textos dos tempos
#[derive(Debug, Deserialize, Serialize)]
pub struct TempoTextoConfig {
    tempo_texto1: String,
    tempo_texto2: String,
    tempo_texto3: String,
    tempo_texto4: String,
    tempo_texto5: String,
    tempo_texto6: String,
    tempo_texto7: String,
    tempo_texto8: String,
    tempo_texto9: String,
}

impl TempoTextoConfig {
    /// Retorna o texto configurado para o índice de tempo especificado
    pub fn get_tempo_texto(&self, index: usize) -> &str {
        match index {
            1 => &self.tempo_texto1,
            2 => &self.tempo_texto2,
            3 => &self.tempo_texto3,
            4 => &self.tempo_texto4,
            5 => &self.tempo_texto5,
            6 => &self.tempo_texto6,
            7 => &self.tempo_texto7,
            8 => &self.tempo_texto8,
            9 => &self.tempo_texto9,
            _ => "",
        }
    }
}

impl Default for TempoTextoConfig {
    fn default() -> Self {
        TempoTextoConfig {
            tempo_texto1: "00:30".to_string(),
            tempo_texto2: "01:00".to_string(),
            tempo_texto3: "01:15".to_string(),
            tempo_texto4: "02:00".to_string(),
            tempo_texto5: "20:00".to_string(),
            tempo_texto6: "00:00".to_string(),
            tempo_texto7: "02:00".to_string(),
            tempo_texto8: "20:00".to_string(),
            tempo_texto9: "00:00".to_string(),
        }
    }
}

/// Configuração dos créditos
#[derive(Debug, Deserialize, Serialize)]
pub struct CreditosConfig {
    credito_texto: String,
    mostrar_creditos: bool,
}

impl CreditosConfig {
    /// Retorna o texto dos créditos
    pub fn get_credito_texto(&self) -> &str {
        &self.credito_texto
    }

    /// Retorna se deve mostrar os créditos
    pub fn mostrar_creditos(&self) -> bool {
        self.mostrar_creditos
    }
}

impl Default for CreditosConfig {
    fn default() -> Self {
        CreditosConfig {
            credito_texto: "Usando configurações padrão, modifique Config.toml para personalizar".to_string(),
            mostrar_creditos: true,
        }
    }
}

/// Configuração dos botões
#[derive(Debug, Deserialize, Serialize)]
pub struct BotoesConfig {
    botao_reset: bool,
    botao_iniciar: bool,
    botao_pausar: bool,
}

impl BotoesConfig {
    /// Retorna se deve mostrar o botão de reset
    pub fn mostrar_botao_reset(&self) -> bool {
        self.botao_reset
    }

    /// Retorna se deve mostrar o botão de iniciar
    pub fn mostrar_botao_iniciar(&self) -> bool {
        self.botao_iniciar
    }

    /// Retorna se deve mostrar o botão de pausar
    pub fn mostrar_botao_pausar(&self) -> bool {
        self.botao_pausar
    }
}

impl Default for BotoesConfig {
    fn default() -> Self {
        BotoesConfig {
            botao_reset: true,
            botao_iniciar: true,
            botao_pausar: true,
        }
    }
}

/// Configuração de debug
#[derive(Debug, Deserialize, Serialize)]
pub struct DebugConfig {
    mostrar_qps: bool,
    mostrar_timer: bool,
}

impl DebugConfig {
    /// Retorna se deve mostrar o QPS (Quadros Por Segundo)
    pub fn mostrar_qps(&self) -> bool {
        self.mostrar_qps
    }

    /// Retorna se deve mostrar o timer
    pub fn mostrar_timer(&self) -> bool {
        self.mostrar_timer
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        DebugConfig {
            mostrar_qps: false,
            mostrar_timer: false,
        }
    }
}

impl RegressivaConfig {
    /// Carrega a configuração do arquivo TOML ou cria um novo com valores padrão
    pub fn load_config() -> Result<RegressivaConfig, Box<dyn std::error::Error>> {
        let config_path = PathBuf::from("Config.toml");

        if !config_path.exists() {
            let default_config = RegressivaConfig::default();
            default_config.save_config(&config_path)?;
            println!("Created default configuration file: {:?}", config_path);
            return Ok(default_config);
        }

        let config_str = fs::read_to_string(&config_path)?;
        let config: RegressivaConfig = toml::from_str(&config_str)?;
        Ok(config)
    }

    fn save_config(&self, config_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(config_path, toml_string)?;
        Ok(())
    }
}