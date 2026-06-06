use rhoiscribe::tools::{BatchEntry, FocusBatchRequest, ToolEngine};

#[test]
fn focus_batch_snapshot_matches_expected_output() {
    let result = ToolEngine::generate_focus_batch(FocusBatchRequest {
        country_tag: "BTA".to_string(),
        tree_id: "BTA_01".to_string(),
        focuses: vec![BatchEntry {
            id: "BTA_01_FS_00".to_string(),
            title: "Stabilize the Cabinet".to_string(),
            description: Some("Establish a stable political baseline.".to_string()),
        }],
        dry_run: true,
        output_root: None,
    })
    .expect("focus dry-run should succeed");

    assert_eq!(result.files.len(), 1);
    assert_eq!(result.files[0].path, "common/national_focus/BTA_01.txt");
    assert_eq!(
        result.files[0].content,
        "focus_tree = {\n\tid = BTA_01\n\tcountry = { factor = 0 modifier = { add = 10 tag = BTA } }\n\tfocus = {\n\t\tid = BTA_01_FS_00\n\t\ticon = GFX_focus_BTA_01_FS_00\n\t\tx = 0\n\t\ty = 0\n\t\tcost = 10\n\t\tcompletion_reward = { add_political_power = 50 }\n\t}\n}\n"
    );
}
