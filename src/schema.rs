table! {
    builds (id) {
        id -> Text,
        ci_build_id -> Nullable<Text>,
        number -> Nullable<Integer>,
        branch_name -> Nullable<Text>,
        status -> Nullable<Text>,
        project_id -> Text,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        user_id -> Nullable<Text>,
        is_running -> Nullable<Bool>,
    }
}

table! {
    projects (id) {
        id -> Text,
        name -> Text,
        main_branch_name -> Text,
        builds_counter -> Integer,
        max_build_allowed -> Integer,
        max_branch_lifetime -> Integer,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        auto_approve_feature -> Bool,
        image_comparison -> Text,
        image_comparison_config -> Text,
    }
}

table! {
    test_runs (id) {
        id -> Text,
        image_name -> Text,
        diff_name -> Nullable<Text>,
        diff_percent -> Nullable<Float>,
        diff_tollerance_percent -> Float,
        pixel_mis_match_count -> Nullable<Integer>,
        status -> Text,
        build_id -> Text,
        test_variation_id -> Nullable<Text>,
        project_id -> Nullable<Text>,
        merge -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        name -> Text,
        browser -> Nullable<Text>,
        device -> Nullable<Text>,
        os -> Nullable<Text>,
        viewport -> Nullable<Text>,
        custom_tags -> Nullable<Text>,
        baseline_name -> Nullable<Text>,
        comment -> Nullable<Text>,
        branch_name -> Text,
        baseline_branch_name -> Nullable<Text>,
        ignore_areas -> Text,
        temp_ignore_areas -> Text,
    }
}

table! {
    test_variations (id) {
        id -> Text,
        name -> Text,
        branch_name -> Text,
        browser -> Text,
        device -> Text,
        os -> Text,
        viewport -> Text,
        custom_tags -> Text,
        baseline_name -> Nullable<Text>,
        ignore_areas -> Text,
        project_id -> Text,
        comment -> Nullable<Text>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    baselines (id) {
        id -> Text,
        baseline_name -> Text,
        test_variation_id -> Text,
        test_run_id -> Nullable<Text>,
        user_id -> Nullable<Text>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Text,
        email -> Text,
        password -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        api_key -> Text,
        is_active -> Bool,
        role -> Text,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}
