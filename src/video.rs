use crate::{actions::Actions, GameState};
use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, HtmlVideoElement};

const CUTSCENE_VIDEO_FILE: &str = "assets/video/cutscene.mp4";
const CUTSCENE_HTML_ID: &str = "cutscene";
const CUTSCENE_NEXT_STATE: GameState = GameState::Menu;

pub struct VideoPlugin;

#[derive(Default, Debug, PartialEq)]
pub enum VideoState {
    Playing,
    #[default]
    Ended,
}

#[derive(Default, Resource, Debug)]
pub struct CutsceneVideo {
    pub src: String,
    pub element_id: String,
    pub is_playing: VideoState,
}

/// This plugin is responsible for the playing video.
/// The video is only played during the State `GameState::CutScene` and is removed when that state is exited
impl Plugin for VideoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CutsceneVideo {
            src: String::from(CUTSCENE_VIDEO_FILE),
            element_id: String::from(CUTSCENE_HTML_ID),
            ..Default::default()
        })
        .add_systems(Startup, setup_web_video)
        //.add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
        .add_systems(OnEnter(GameState::PlayingCutScene), play_video)
        .add_systems(
            Update,
            check_video
                .after(play_video)
                .run_if(in_state(GameState::PlayingCutScene)),
        )
        .add_systems(OnExit(GameState::PlayingCutScene), cleanup_video)
        // TODO Debug only
        .add_systems(
            Update,
            debug_play_cutscene.run_if(in_state(GameState::Playing)),
        );
    }
}

/// This system initializes the video for web and preloads it for quick playback later.
fn setup_web_video(cutscene: Res<CutsceneVideo>) {
    //todo!("setup_video");

    let window = web_sys::window().expect("no window!");
    let document = window.document().expect("no document!");

    // get canvas element
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("bevy")
        .expect("there to be a canvas element named 'bevy'")
        .dyn_into()
        .expect("element to be a canvas");

    // create <video> element
    let video = document
        .create_element("video")
        .expect("can't create video element")
        .dyn_into::<HtmlVideoElement>()
        .expect("can't cast to HTMLVideoElement");

    // configure <video>
    video.set_id("cutscene");
    video.set_src(&cutscene.src);
    if let Some(error) = video.set_attribute("type", "video/mp4").err() {
        console_log!("Error: {:#?}", error);
    }
    video.set_preload("auto");
    video.set_hidden(true);
    video
        .style()
        .set_property("position", "absolute")
        .expect("can't set position");
    video
        .style()
        .set_property("z-index", "2")
        .expect("can't set z-index");

    resize_video(&canvas, &video);

    // append <video>
    document
        .body()
        .expect("can't get body")
        .append_child(&video)
        .expect("can't append video");

    // attach listener for when the window is resized
    let closure = Closure::wrap(Box::new(move |event: web_sys::UiEvent| {
        //console_log!(format!("event type: {}", e.type_()).into());
        if event.type_() == "resize" {
            resize_video(&canvas, &video);
        }
    }) as Box<dyn FnMut(_)>);
    window
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .expect("can't add resize listener");
    closure.forget();
}

fn cleanup_video(mut state: ResMut<NextState<GameState>>) {
    let video = get_html_video_element(&String::from(CUTSCENE_HTML_ID));

    video.set_hidden(true);

    // now that video has ended, exit video playback state
    state.set(CUTSCENE_NEXT_STATE);
}

fn resize_video(canvas: &HtmlCanvasElement, video: &HtmlVideoElement) {
    let rect = canvas.get_bounding_client_rect();

    video
        .style()
        .set_property("left", &format!("{}px", rect.left()))
        .expect("to set left");
    video
        .style()
        .set_property("top", &format!("{}px", rect.top()))
        .expect("to set top");
    video
        .style()
        .set_property("width", &format!("{}px", rect.width()))
        .expect("to set width");
    video
        .style()
        .set_property("height", &format!("{}px", rect.height()))
        .expect("to set height");

    console_log!("resize!");
}

pub fn play_video(video_resource: Res<CutsceneVideo>) {
    let video = get_html_video_element(&video_resource.element_id);

    console_log!("{:#?}", video);

    video.set_hidden(false);
    if let Some(error) = video.play().err() {
        console_log!("{:#?}", error);
    }
}

/// This will check the <video> element to determine if it is playing or
/// ended
fn check_video(mut video_resource: ResMut<CutsceneVideo>, mut state: ResMut<NextState<GameState>>) {
    let video = get_html_video_element(&video_resource.element_id);

    video_resource.is_playing = match video.ended() {
        true => VideoState::Ended,
        false => VideoState::Playing,
    };

    if video_resource.is_playing == VideoState::Ended {
        state.set(CUTSCENE_NEXT_STATE);
    }

    console_log!(
        "video: {:#?}\nvideo_resource.is_playing: {:#?}",
        video,
        video_resource.is_playing
    );
}

fn get_html_video_element(id: &str) -> HtmlVideoElement {
    window()
        .expect("can't get window")
        .document()
        .expect("can't get document")
        .get_element_by_id(id)
        .expect("can't get video element")
        .dyn_into()
        .expect("dyn into cast to video element")
}

/// TODO Debug only
fn debug_play_cutscene(mut state: ResMut<NextState<GameState>>, actions: Res<Actions>) {
    if actions.debug_play_cutscene {
        state.set(GameState::PlayingCutScene);
    }
}
