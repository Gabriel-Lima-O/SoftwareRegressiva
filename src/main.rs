use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use serde_derive::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use toml;

// Estrutura principal de configuração
#[derive(Debug, Deserialize)]
struct RegressivaConfig {
    #[serde(rename = "tempo")]
    tempo: TempoConfig,

    #[serde(rename = "tempo_texto")]
    tempo_texto: TempoTextoConfig,

    #[serde(rename = "creditos")]
    creditos: CreditosConfig,

    #[serde(rename = "botoes")]
    botoes: BotoesConfig,

    #[serde(rename = "debug")]
    debug: DebugConfig,

    #[serde(rename = "fontes")]
    fontes: FontesConfig,

    #[serde(rename = "boas_vindas")]
    boas_vindas: BoasVindasConfig,
}

#[derive(Debug, Deserialize)]
struct BoasVindasConfig {
    mostrar_boas_vindas: bool,
}

// Configuração das fontes
#[derive(Debug, Deserialize)]
struct FontesConfig {
    fonte: String,
    fonte_gigante: u16,
    fonte_media: u16,
    fonte_grande: u16,
    fonte_pequena: u16,
}

// Configuração dos tempos
#[derive(Debug, Deserialize)]
struct TempoConfig {
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

// Configuração dos textos dos tempos
#[derive(Debug, Deserialize)]
struct TempoTextoConfig {
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

// Configuração dos créditos
#[derive(Debug, Deserialize)]
struct CreditosConfig {
    credito_texto: String,
    mostrar_creditos: bool,
}

// Configuração dos botões
#[derive(Debug, Deserialize)]
struct BotoesConfig {
    botao_reset: bool,
    botao_iniciar: bool,
    botao_pausar: bool,
}

// Configuração de debug
#[derive(Debug, Deserialize)]
struct DebugConfig {
    mostrar_qps: bool,
    mostrar_timer: bool,
}

// Função para renderizar texto no canvas
fn render_text<'a>(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &Font,
    text: &str,
    color: Color,
    text_rect: Rect,
) {
    if text.is_empty() {
        return;
    }

    let surface = font.render(text).blended(color).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let text_width = surface.width() as i32;
    let text_height = surface.height() as i32;

    let centered_rect = Rect::new(
        text_rect.x() + (text_rect.width() as i32 - text_width) / 2,
        text_rect.y() + (text_rect.height() as i32 - text_height) / 2,
        text_width as u32,
        text_height as u32,
    );

    canvas.copy(&texture, None, Some(centered_rect)).unwrap();
}

// Função para lidar com cliques do mouse
fn handle_mouse_click(
    x: i32,
    y: i32,
    config: &RegressivaConfig,
    buttons: &ButtonPositions,
    is_running: &mut bool,
    countdown_duration: &mut Duration,
    start_time: &mut Instant,
) {
    if buttons.start_button.contains_point((x, y)) && config.botoes.botao_iniciar {
        *is_running = true;
        *start_time = Instant::now();
    } else if buttons.pause_button.contains_point((x, y)) && config.botoes.botao_pausar {
        *is_running = false;
    } else if buttons.reset_button.contains_point((x, y)) && config.botoes.botao_reset {
        *is_running = false;
        *countdown_duration = Duration::new(config.tempo.tempo_reset, 0);
    } else if buttons.close_button.contains_point((x, y)) {
        std::process::exit(0);
    } else {
        let button_durations = [
            (buttons.botao_1, config.tempo.tempo1),
            (buttons.botao_2, config.tempo.tempo2),
            (buttons.botao_3, config.tempo.tempo3),
            (buttons.botao_4, config.tempo.tempo4),
            (buttons.botao_5, config.tempo.tempo5),
            (buttons.botao_6, config.tempo.tempo6),
            (buttons.botao_7, config.tempo.tempo7),
            (buttons.botao_8, config.tempo.tempo8),
            (buttons.botao_9, config.tempo.tempo9),
        ];

        for (button, duration) in button_durations.iter() {
            if button.contains_point((x, y)) {
                *countdown_duration = Duration::new(*duration, 0);
                *is_running = false;
                break;
            }
        }
    }
}

// Função para lidar com entrada de texto
fn handle_text_input(
    text: String,
    input_text: &mut String,
    mouse_state: &sdl2::mouse::MouseState,
    input_rect: Rect,
) {
    if input_rect.contains_point((mouse_state.x(), mouse_state.y())) {
        if input_text.len() == 2 && !input_text.contains(':') {
            input_text.push(':');
        }
        input_text.push_str(&text);

        if input_text.len() > 5 {
            input_text.truncate(5);
        }
    }
}

// Função para lidar com a tecla de backspace
fn handle_backspace(input_text: &mut String) {
    input_text.pop();
    if input_text.len() == 2 && input_text.contains(':') {
        input_text.pop();
    }
}

// Função para lidar com a tecla Enter
fn handle_enter(input_text: &mut String, is_running: &mut bool, countdown_duration: &mut Duration) {
    if let Some((minutes, seconds)) = input_text.split_once(':') {
        if let (Ok(min), Ok(sec)) = (minutes.parse::<u64>(), seconds.parse::<u64>()) {
            *is_running = false;
            *countdown_duration = Duration::new(min * 60 + sec, 0);
            input_text.clear();
        }
    }
}

// Estrutura para armazenar as posições dos botões
struct ButtonPositions {
    start_button: Rect,
    pause_button: Rect,
    reset_button: Rect,
    close_button: Rect,
    botao_1: Rect,
    botao_2: Rect,
    botao_3: Rect,
    botao_4: Rect,
    botao_5: Rect,
    botao_6: Rect,
    botao_7: Rect,
    botao_8: Rect,
    botao_9: Rect,
}

fn boas_vindas() {
    let greeting = r#"
    Bem-vindo ao programa regressiva!

    Para personalizar as configurações, por favor, edite o arquivo de configuração. Aqui está um guia para ajudá-lo:

    [tempo] # Define os valores de tempo em segundos
    - tempo1 a tempo9: Defina os valores de tempo (em segundos).
    - tempo_reset: Defina o tempo (em segundos) para reiniciar.

    [tempo_texto] # Rótulos para os botões
    - tempo_texto1 a tempo_texto9: Defina o texto de exibição para cada botão.

    [creditos] # Créditos do programa
    - credito_texto: Defina o texto de crédito.
    - mostrar_creditos: Mostrar ou ocultar créditos (verdadeiro/falso).

    [botoes] # Controle de visibilidade dos botões
    - botao_iniciar: Mostrar/Ocultar botão de iniciar.
    - botao_pausar: Mostrar/Ocultar botão de pausar.
    - botao_reset: Mostrar/Ocultar botão de reiniciar.

    [debug] # Configurações de depuração
    - mostrar_qps: Mostrar informações de QPS (quadros por segundo) (verdadeiro/falso).
    - mostrar_timer: Mostrar informações do temporizador (verdadeiro/falso).

    [fontes] # Configurações de fonte
    - fonte: Defina o arquivo de fonte.
    - fonte_gigante, fonte_grande, fonte_media, fonte_pequena: Defina os tamanhos das fontes.

    [boas_vindas]
    - mostrar_boas_vindas: Mostrar/Ocultar mensagem de boas-vindas (verdadeiro/falso).
    
    Por favor, certifique-se de salvar suas alterações antes de iniciar o programa!

    Para encerrar o programa, feche a janela do console.
    
    "#;

    println!("{}", greeting);
}

fn to_u32(value: f64) -> u32 {
    value.round() as u32
}

fn update_countdown(
    start_time: &mut Instant,
    countdown_duration: &mut Duration,
    is_running: &mut bool,
) {
    if *is_running {
        let elapsed = start_time.elapsed();
            if *countdown_duration > elapsed {
                *countdown_duration -= elapsed;
            } else {
                *countdown_duration = Duration::new(0, 0);
                *is_running = false;
            }
        } 
        *start_time = Instant::now();
    }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Carrega a configuração do arquivo TOML
    let config_str = fs::read_to_string("Config.toml")?;
    let config: RegressivaConfig = toml::de::from_str(&config_str)?;

    if config.boas_vindas.mostrar_boas_vindas {
        boas_vindas();
    }
    // Inicializa o SDL e seus subsistemas
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init()?;

    // Cria a janela do timer
    let window_timer = video_subsystem
        .window("Regressiva", 1280, 720)
        .position_centered()
        .borderless()
        .build()?;

    let mut canvas_timer = window_timer.into_canvas().build()?;
    canvas_timer.set_draw_color(Color::BLACK);

    // Cria a janela dos botões
    let window_buttons = video_subsystem
        .window("Controle", 1280, 720)
        .position_centered()
        .borderless()
        .build()?;

    let mut canvas_buttons = window_buttons.into_canvas().build()?;
    let custom_background_color = Color::RGB(9, 61, 83);
    let custom_red_color = Color::RGBA(100, 0, 0, 150);
    let custom_green_color = Color::RGBA(0, 100, 0, 150);
    let custom_blue_color = Color::RGBA(0, 0, 100, 150);
    canvas_buttons.set_draw_color(custom_background_color);

    // Carrega as fontes
    let fonte_gigantic = ttf_context
        .load_font(
            "./fonts/".to_owned() + &config.fontes.fonte,
            config.fontes.fonte_gigante,
        )
        .map_err(|e| format!("Failed to load font: {:?}", e))?;
    let font_medium = ttf_context
        .load_font(
            "./fonts/".to_owned() + &config.fontes.fonte,
            config.fontes.fonte_media,
        )
        .map_err(|e| format!("Failed to load font: {:?}", e))?;
    let fonte_large = ttf_context
        .load_font(
            "./fonts/".to_owned() + &config.fontes.fonte,
            config.fontes.fonte_grande,
        )
        .map_err(|e| format!("Failed to load font: {:?}", e))?;
    let font_small = ttf_context
        .load_font(
            "./fonts/".to_owned() + &config.fontes.fonte,
            config.fontes.fonte_pequena,
        )
        .map_err(|e| format!("Failed to load font: {:?}", e))?;

    // Inicializa variáveis de controle
    let mut start_time = Instant::now();
    let mut countdown_duration = Duration::new(30, 0);
    let mut is_running = false;
    let mut input_text = String::new();
    let placeholder_text = "Clique para editar".to_string();

    // Variáveis para cálculo de FPS
    let mut last_fps_update = Instant::now();
    let mut frame_count = 0;
    let mut fps = 0.0;

    // Define as posições dos botões
    let buttons = ButtonPositions {
        start_button: Rect::new(
            to_u32(50.0 * 3.2) as i32,
            to_u32(50.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        pause_button: Rect::new(
            to_u32(160.0 * 3.2) as i32,
            to_u32(50.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        reset_button: Rect::new(
            to_u32(270.0 * 3.2) as i32,
            to_u32(50.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        close_button: Rect::new(1230, 10, 40, 40),
        botao_1: Rect::new(
            to_u32(50.0 * 3.2) as i32,
            to_u32(120.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        botao_2: Rect::new(
            to_u32(160.0 * 3.2) as i32,
            to_u32(120.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        botao_3: Rect::new(
            to_u32(270.0 * 3.2) as i32,
            to_u32(120.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        botao_4: Rect::new(
            to_u32(50.0 * 3.2) as i32,
            to_u32(190.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        botao_5: Rect::new(
            to_u32(160.0 * 3.2) as i32,
            to_u32(190.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        botao_6: Rect::new(
            to_u32(270.0 * 3.2) as i32,
            to_u32(190.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        botao_7: Rect::new(
            to_u32(50.0 * 3.2) as i32,
            to_u32(260.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        botao_8: Rect::new(
            to_u32(160.0 * 3.2) as i32,
            to_u32(260.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
        botao_9: Rect::new(
            to_u32(270.0 * 3.2) as i32,
            to_u32(260.0 * 1.44) as i32,
            to_u32(100.0 * 3.2),
            to_u32(50.0 * 1.44),
        ),
    };

    // Define a área de entrada de texto
    let input_rect = Rect::new(
        to_u32(50.0 * 3.2) as i32,
        to_u32(330.0 * 1.44) as i32,
        to_u32(300.0 * 3.2),
        to_u32(50.0 * 1.44),
    );
    let small_timer_rect = Rect::new(
        to_u32(50.0 * 3.2) as i32,
        to_u32(390.0 * 1.44) as i32,
        to_u32(300.0 * 3.2),
        to_u32(50.0 * 1.44),
    );


    let mut event_pump = sdl_context.event_pump()?;

    // Variáveis adicionadas antes do loop
    let mut cursor_visible = true;
    let cursor_toggle_duration = Duration::from_millis(500);
    let mut last_cursor_toggle = Instant::now();
    let mut is_input_focused = false;

    // Loop principal
    'running: loop {
        let mouse_state = event_pump.mouse_state();
        let now = Instant::now();

        // Processa eventos
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        handle_mouse_click(
                            x,
                            y,
                            &config,
                            &buttons,
                            &mut is_running,
                            &mut countdown_duration,
                            &mut start_time,
                        );

                        // Verifica se o clique foi dentro da área de entrada de texto
                        if input_rect.contains_point((x, y)) {
                            is_input_focused = true;
                        } else {
                            is_input_focused = false;
                        }
                    }
                }
                Event::TextInput { text, .. } => {
                    if is_input_focused {
                        handle_text_input(text, &mut input_text, &mouse_state, input_rect);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    if is_input_focused {
                        handle_backspace(&mut input_text);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::KP_ENTER),
                    ..
                } => {
                    if is_input_focused {
                        handle_enter(&mut input_text, &mut is_running, &mut countdown_duration);
                    }
                }
                _ => {}
            }
        }

        // Atualiza a visibilidade do cursor apenas se o campo de entrada estiver focado
        if is_input_focused && now.duration_since(last_cursor_toggle) >= cursor_toggle_duration {
            cursor_visible = !cursor_visible;
            last_cursor_toggle = now;
        }

        // Atualiza a contagem regressiva
        update_countdown(
            &mut start_time,
            &mut countdown_duration,
            &mut is_running,
        );

        // Calcula o tempo restante
        let remaining_secs = countdown_duration.as_secs();
        let minutes = remaining_secs / 60;
        let seconds = remaining_secs % 60;

        // Define a cor do timer
        let timer_color = if remaining_secs <= 10 && remaining_secs > 0 {
            if seconds % 2 == 0 {
                Color::RED
            } else {
                Color::WHITE
            }
        } else {
            Color::WHITE
        };

        // Renderiza o timer na janela principal
        canvas_timer.set_draw_color(custom_background_color);
        canvas_timer.clear();
        let timer_text = format!("{:02}:{:02}", minutes, seconds);
        let text_surface = fonte_gigantic.render(&timer_text).blended(timer_color)?;
        let text_width = text_surface.width();
        let text_height = text_surface.height();
        let text_rect = Rect::new(
            640 - text_width as i32 / 2,
            360 - text_height as i32 / 2,
            text_width,
            text_height,
        );
        render_text(
            &mut canvas_timer,
            &fonte_gigantic,
            &timer_text,
            timer_color,
            text_rect,
        );
        canvas_timer.present();

        // Renderiza os botões e outros elementos na janela de controle
        canvas_buttons.set_draw_color(custom_background_color);
        canvas_buttons.clear();

        // Renderiza o botão de iniciar
        if config.botoes.botao_iniciar == true {
            canvas_buttons.set_draw_color(custom_green_color);
            canvas_buttons.fill_rect(buttons.start_button)?;
            render_text(
                &mut canvas_buttons,
                &fonte_large,
                "Iniciar",
                Color::WHITE,
                buttons.start_button,
            );
        }

        // Renderiza o botão de pausar
        if config.botoes.botao_pausar == true {
            canvas_buttons.set_draw_color(custom_blue_color);
            canvas_buttons.fill_rect(buttons.pause_button)?;
            render_text(
                &mut canvas_buttons,
                &fonte_large,
                "Pausar",
                Color::WHITE,
                buttons.pause_button,
            );
        }

        // Renderiza o botão de reset
        if config.botoes.botao_reset {
            canvas_buttons.set_draw_color(custom_red_color);
            canvas_buttons.fill_rect(buttons.reset_button)?;
            render_text(
                &mut canvas_buttons,
                &fonte_large,
                "Reset",
                Color::WHITE,
                buttons.reset_button,
            );
        }

        // Renderiza o botão de fechar
        canvas_buttons.set_draw_color(Color::RGB(9, 61, 83));
        canvas_buttons.fill_rect(buttons.close_button)?;
        render_text(
            &mut canvas_buttons,
            &fonte_large,
            "X",
            Color::WHITE,
            buttons.close_button,
        );

        // Renderiza os botões de tempo
        let button_texts = [
            (buttons.botao_7, &config.tempo_texto.tempo_texto7),
            (buttons.botao_8, &config.tempo_texto.tempo_texto8),
            (buttons.botao_9, &config.tempo_texto.tempo_texto9),
            (buttons.botao_1, &config.tempo_texto.tempo_texto1),
            (buttons.botao_2, &config.tempo_texto.tempo_texto2),
            (buttons.botao_3, &config.tempo_texto.tempo_texto3),
            (buttons.botao_4, &config.tempo_texto.tempo_texto4),
            (buttons.botao_5, &config.tempo_texto.tempo_texto5),
            (buttons.botao_6, &config.tempo_texto.tempo_texto6),
        ];

        for (button, text) in button_texts.iter() {
            canvas_buttons.set_draw_color(Color::WHITE);
            canvas_buttons.fill_rect(*button)?;
            render_text(
                &mut canvas_buttons,
                &font_small,
                text,
                Color::BLACK,
                *button,
            );
        }

        // Renderiza a área de entrada de texto com o cursor
        if is_input_focused {
            if cursor_visible {
                if input_text.is_empty() {
                    // Renderiza apenas o cursor
                    render_text(
                        &mut canvas_buttons,
                        &font_small,
                        "|",
                        Color::WHITE,
                        input_rect,
                    );
                } else {
                    // Renderiza o texto com o cursor
                    let display_text = format!("{}|", input_text);
                    render_text(
                        &mut canvas_buttons,
                        &font_small,
                        &display_text,
                        Color::WHITE,
                        input_rect,
                    );
                }
            } else {
                // Renderiza apenas o texto sem o cursor
                render_text(
                    &mut canvas_buttons,
                    &font_small,
                    &input_text,
                    Color::WHITE,
                    input_rect,
                );
            }
        } else if input_text.is_empty() {
            // Renderiza o placeholder se o campo não estiver focado e estiver vazio
            render_text(
                &mut canvas_buttons,
                &fonte_large,
                &placeholder_text,
                Color::GRAY,
                input_rect,
            );
        } else {
            // Renderiza o texto sem o cursor
            render_text(
                &mut canvas_buttons,
                &font_small,
                &input_text,
                Color::WHITE,
                input_rect,
            );
        }
        canvas_buttons.set_draw_color(Color::WHITE);
        canvas_buttons.draw_rect(input_rect)?;

        // Renderiza o timer pequeno na janela de controle
        render_text(
            &mut canvas_buttons,
            &fonte_large,
            &timer_text,
            timer_color,
            small_timer_rect,
        );

        // Renderiza os créditos, se configurado
        if config.creditos.mostrar_creditos == true {
            let tips_text = &config.creditos.credito_texto;
            render_text(
                &mut canvas_buttons,
                &font_medium,
                tips_text,
                Color::WHITE,
                Rect::new(85, 680, 300, 30),
            );
        }

        // Calcula e renderiza o FPS, se configurado
        frame_count += 1;
        if now.duration_since(last_fps_update).as_secs() >= 1 {
            fps = frame_count as f64 / now.duration_since(last_fps_update).as_secs_f64();
            frame_count = 0;
            last_fps_update = now;
        }

        if config.debug.mostrar_timer == true {
            print!("{}", timer_text); // Using println! for automatic newline and flush
        }

        if config.debug.mostrar_qps == true {
            let fps_text = format!("QPS: {:.2}", fps);
            print!(" | {}", fps_text); // Using println! for automatic newline and flush
            render_text(
                &mut canvas_buttons,
                &font_medium,
                &fps_text,
                Color::WHITE,
                Rect::new(10, 10, 100, 30),
            );
        }
        io::stdout().flush().unwrap();
        canvas_buttons.present();

        // Limita a taxa de atualização
        std::thread::sleep(Duration::from_millis(32));
    }
    Ok(())
}
