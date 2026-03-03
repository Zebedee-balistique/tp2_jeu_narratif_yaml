use tp_2_jeu_narratif_yaml::story::{Story};

#[test]
fn load_story_test() {
    let story = Story::load_story("story.yaml").expect("fichier chargé");

    assert_eq!(story.start_scene, "entrance".to_string());
    assert_eq!(story.initial_hp, 10);
}

#[test]
fn load_story_test() {
    let story = Story::load_story("story.yaml").expect("fichier chargé");

    assert_eq!(story.start_scene, "entrance".to_string());
    assert_eq!(story.initial_hp, 10);
}

