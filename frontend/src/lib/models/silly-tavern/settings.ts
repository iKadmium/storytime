export interface SettingsResponse {
    settings: string;
}

export interface Settings {
    firstRun: boolean;
    accountStorage: AccountStorage;
    currentVersion: string;
    username: string;
    active_character: string;
    active_group: null;
    user_avatar: string;
    amount_gen: number;
    max_context: number;
    main_api: string;
    world_info_settings: WorldInfoSettings;
    textgenerationwebui_settings: TextgenerationwebuiSettings;
    swipes: boolean;
    horde_settings: HordeSettings;
    power_user: PowerUser;
    extension_settings: ExtensionSettings;
    tags: Tag[];
    tag_map: TagMap;
    nai_settings: NaiSettings;
    kai_settings: KaiSettings;
    oai_settings: OaiSettings;
    background: Background;
    proxies: Proxy[];
    selected_proxy: Proxy;
}

export interface AccountStorage {
    __migrated: string;
    NavOpened: string;
    StoryStringValidationCache: string;
    LNavOpened: string;
}

export interface Background {
    name: string;
    url: string;
    fitting: string;
    animation: boolean;
}

export interface ExtensionSettings {
    apiUrl: string;
    apiKey: string;
    autoConnect: boolean;
    notifyUpdates: boolean;
    disabledExtensions: any[];
    expressionOverrides: any[];
    memory: Memory;
    note: Note;
    caption: Caption;
    expressions: Expressions;
    connectionManager: ConnectionManager;
    dice: CharacterAttachments;
    regex: any[];
    regex_presets: any[];
    character_allowed_regex: any[];
    tts: TTS;
    sd: SD;
    chromadb: CharacterAttachments;
    translate: Translate;
    objective: Objective;
    quickReply: QuickReply;
    randomizer: Randomizer;
    speech_recognition: SpeechRecognition;
    rvc: Rvc;
    hypebot: CharacterAttachments;
    vectors: CharacterAttachments;
    variables: Variables;
    attachments: any[];
    character_attachments: CharacterAttachments;
    disabled_attachments: any[];
    gallery: Gallery;
    cfg: CFG;
    quickReplyV2: QuickReplyV2;
}

export interface Caption {
    refine_mode: boolean;
    source: string;
    multimodal_api: string;
    multimodal_model: string;
    prompt: string;
    template: string;
    show_in_chat: boolean;
}

export interface CFG {
    global: Global;
    chara: any[];
}

export interface Global {
    guidance_scale: number;
    negative_prompt: string;
}

export interface CharacterAttachments {
}

export interface ConnectionManager {
    selectedProfile: string;
    profiles: Profile[];
}

export interface Profile {
    id: string;
    mode: string;
    exclude: any[];
    api: string;
    preset: string;
    "api-url": string;
    sysprompt: string;
    "sysprompt-state": string;
    context: string;
    "instruct-state": string;
    tokenizer: string;
    "stop-strings": string;
    "start-reply-with": string;
    "reasoning-template": string;
    name: string;
}

export interface Expressions {
    showDefault: boolean;
    api: number;
    llmPrompt: string;
    allowMultiple: boolean;
    promptType: string;
    custom: any[];
}

export interface Gallery {
    folders: CharacterAttachments;
    sort: string;
}

export interface Memory {
    minLongMemory: number;
    maxLongMemory: number;
    longMemoryLength: number;
    shortMemoryLength: number;
    minShortMemory: number;
    maxShortMemory: number;
    shortMemoryStep: number;
    longMemoryStep: number;
    repetitionPenaltyStep: number;
    repetitionPenalty: number;
    maxRepetitionPenalty: number;
    minRepetitionPenalty: number;
    temperature: number;
    minTemperature: number;
    maxTemperature: number;
    temperatureStep: number;
    lengthPenalty: number;
    minLengthPenalty: number;
    maxLengthPenalty: number;
    lengthPenaltyStep: number;
    memoryFrozen: boolean;
    source: string;
    prompt: string;
    promptWords: number;
    promptMinWords: number;
    promptMaxWords: number;
    promptWordsStep: number;
    promptInterval: number;
    promptMinInterval: number;
    promptMaxInterval: number;
    promptIntervalStep: number;
    template: string;
    position: number;
    depth: number;
    promptForceWords: number;
    promptForceWordsStep: number;
    promptMinForceWords: number;
    promptMaxForceWords: number;
    SkipWIAN: boolean;
    role: number;
    scan: boolean;
    overrideResponseLength: number;
    overrideResponseLengthMin: number;
    overrideResponseLengthMax: number;
    overrideResponseLengthStep: number;
    maxMessagesPerRequest: number;
    maxMessagesPerRequestMin: number;
    maxMessagesPerRequestMax: number;
    maxMessagesPerRequestStep: number;
    prompt_builder: number;
}

export interface Note {
    default: string;
    chara: any[];
    wiAddition: any[];
    defaultPosition: number;
    defaultDepth: number;
    defaultInterval: number;
    defaultRole: number;
}

export interface Objective {
    customPrompts: CustomPrompts;
}

export interface CustomPrompts {
    default: Default;
}

export interface Default {
    createTask: string;
    checkTaskCompleted: string;
    currentTask: string;
}

export interface QuickReply {
    quickReplyEnabled: boolean;
    numberOfSlots: number;
    quickReplySlots: QuickReplySlot[];
}

export interface QuickReplySlot {
    mes: string;
    label: string;
    enabled: boolean;
}

export interface QuickReplyV2 {
    isEnabled: boolean;
    isCombined: boolean;
    isPopout: boolean;
    config: Config;
    characterConfigs: CharacterConfigs;
}

export interface CharacterConfigs {
    "default_Assistant.png": Config;
}

export interface Config {
    setList: SetList[];
}

export interface SetList {
    set: string;
    isVisible: boolean;
}

export interface Randomizer {
    controls: any[];
    fluctuation: number;
    enabled: boolean;
}

export interface Rvc {
    enabled: boolean;
    model: string;
    pitchOffset: number;
    pitchExtraction: string;
    indexRate: number;
    filterRadius: number;
    rmsMixRate: number;
    protect: number;
    voicMapText: string;
    voiceMap: CharacterAttachments;
}

export interface SD {
    scale_min: number;
    scale_max: number;
    scale_step: number;
    scale: number;
    steps_min: number;
    steps_max: number;
    steps_step: number;
    steps: number;
    dimension_min: number;
    dimension_max: number;
    dimension_step: number;
    width: number;
    height: number;
    prompt_prefix: string;
    negative_prompt: string;
    sampler: string;
    model: string;
    restore_faces: boolean;
    enable_hr: boolean;
    horde: boolean;
    horde_nsfw: boolean;
    horde_karras: boolean;
    refine_mode: boolean;
    prompts: Prompts;
    character_prompts: CharacterAttachments;
    source: string;
    scheduler: string;
    vae: string;
    seed: number;
    adetailer_face: boolean;
    horde_sanitize: boolean;
    interactive_mode: boolean;
    multimodal_captioning: boolean;
    snap: boolean;
    free_extend: boolean;
    function_tool: boolean;
    auto_url: string;
    auto_auth: string;
    vlad_url: string;
    vlad_auth: string;
    drawthings_url: string;
    drawthings_auth: string;
    hr_upscaler: string;
    hr_scale: number;
    hr_scale_min: number;
    hr_scale_max: number;
    hr_scale_step: number;
    denoising_strength: number;
    denoising_strength_min: number;
    denoising_strength_max: number;
    denoising_strength_step: number;
    hr_second_pass_steps: number;
    hr_second_pass_steps_min: number;
    hr_second_pass_steps_max: number;
    hr_second_pass_steps_step: number;
    clip_skip_min: number;
    clip_skip_max: number;
    clip_skip_step: number;
    clip_skip: number;
    novel_anlas_guard: boolean;
    novel_sm: boolean;
    novel_sm_dyn: boolean;
    novel_decrisper: boolean;
    novel_variety_boost: boolean;
    openai_style: string;
    openai_quality: string;
    style: string;
    styles: Style[];
    comfy_url: string;
    comfy_workflow: string;
    pollinations_enhance: boolean;
    wand_visible: boolean;
    command_visible: boolean;
    interactive_visible: boolean;
    tool_visible: boolean;
    stability_style_preset: string;
    bfl_upsampling: boolean;
    google_api: string;
    google_enhance: boolean;
    character_negative_prompts: CharacterAttachments;
}

export interface Prompts {
    "0": string;
    "1": string;
    "2": string;
    "3": string;
    "4": string;
    "5": string;
    "7": string;
    "8": string;
    "9": string;
    "10": string;
    "11": string;
    "-1": string;
    "-2": string;
}

export interface Style {
    name: string;
    negative: string;
    prefix: string;
}

export interface SpeechRecognition {
    currentProvider: string;
    messageMode: string;
    messageMappingText: string;
    messageMapping: any[];
    messageMappingEnabled: boolean;
    None: CharacterAttachments;
}

export interface Translate {
    target_language: string;
    internal_language: string;
    provider: string;
    auto_mode: string;
    deepl_endpoint: string;
}

export interface TTS {
    voiceMap: string;
    ttsEnabled: boolean;
    currentProvider: string;
    auto_generation: boolean;
    ElevenLabs: CharacterAttachments;
    System: CharacterAttachments;
    narrate_user: boolean;
    playback_rate: number;
    multi_voice_enabled: boolean;
    Chatterbox: Chatterbox;
    enabled: boolean;
}

export interface Chatterbox {
    provider_endpoint: string;
    voice_mode: string;
    predefined_voice: string;
    reference_voice: string;
    temperature: number;
    exaggeration: number;
    cfg_weight: number;
    seed: number;
    speed_factor: number;
    language: string;
    split_text: boolean;
    chunk_size: number;
    output_format: string;
    voiceMap: Record<string, string>;
}

export interface Variables {
    global: CharacterAttachments;
}

export interface HordeSettings {
    models: any[];
    auto_adjust_response_length: boolean;
    auto_adjust_context_length: boolean;
    trusted_workers_only: boolean;
}

export interface KaiSettings {
    temp: number;
    rep_pen: number;
    rep_pen_range: number;
    top_p: number;
    min_p: number;
    top_a: number;
    top_k: number;
    typical: number;
    tfs: number;
    rep_pen_slope: number;
    streaming_kobold: boolean;
    sampler_order: number[];
    mirostat: number;
    mirostat_tau: number;
    mirostat_eta: number;
    use_default_badwordsids: boolean;
    grammar: string;
    seed: number;
    api_server: string;
    preset_settings: string;
    extensions: CharacterAttachments;
}

export interface NaiSettings {
    temperature: number;
    repetition_penalty: number;
    repetition_penalty_range: number;
    repetition_penalty_slope: number;
    repetition_penalty_frequency: number;
    repetition_penalty_presence: number;
    tail_free_sampling: number;
    top_k: number;
    top_p: number;
    top_a: number;
    typical_p: number;
    min_p: number;
    math1_temp: number;
    math1_quad: number;
    math1_quad_entropy_scale: number;
    min_length: number;
    model_novel: string;
    preset_settings_novel: string;
    streaming_novel: boolean;
    preamble: string;
    banned_tokens: string;
    order: number[];
    logit_bias: any[];
    extensions: CharacterAttachments;
}

export interface OaiSettings {
    preset_settings_openai: string;
    temp_openai: number;
    freq_pen_openai: number;
    pres_pen_openai: number;
    top_p_openai: number;
    top_k_openai: number;
    min_p_openai: number;
    top_a_openai: number;
    repetition_penalty_openai: number;
    stream_openai: boolean;
    openai_max_context: number;
    openai_max_tokens: number;
    wrap_in_quotes: boolean;
    prompts: Prompt[];
    prompt_order: PromptOrder[];
    send_if_empty: string;
    impersonation_prompt: string;
    new_chat_prompt: string;
    new_group_chat_prompt: string;
    new_example_chat_prompt: string;
    continue_nudge_prompt: string;
    bias_preset_selected: string;
    bias_presets: BiasPresets;
    wi_format: string;
    group_nudge_prompt: string;
    scenario_format: string;
    personality_format: string;
    openai_model: string;
    claude_model: string;
    google_model: string;
    vertexai_model: string;
    ai21_model: string;
    mistralai_model: string;
    cohere_model: string;
    perplexity_model: string;
    groq_model: string;
    electronhub_model: string;
    nanogpt_model: string;
    deepseek_model: string;
    aimlapi_model: string;
    xai_model: string;
    pollinations_model: string;
    cometapi_model: string;
    moonshot_model: string;
    fireworks_model: string;
    azure_base_url: string;
    azure_deployment_name: string;
    azure_api_version: string;
    azure_openai_model: string;
    custom_model: string;
    custom_url: string;
    custom_include_body: string;
    custom_exclude_body: string;
    custom_include_headers: string;
    openrouter_model: string;
    openrouter_use_fallback: boolean;
    openrouter_group_models: boolean;
    openrouter_sort_models: string;
    openrouter_providers: any[];
    openrouter_allow_fallbacks: boolean;
    openrouter_middleout: string;
    reverse_proxy: string;
    chat_completion_source: string;
    max_context_unlocked: boolean;
    show_external_models: boolean;
    proxy_password: string;
    assistant_prefill: string;
    assistant_impersonation: string;
    claude_use_sysprompt: boolean;
    use_makersuite_sysprompt: boolean;
    vertexai_auth_mode: string;
    vertexai_region: string;
    vertexai_express_project_id: string;
    squash_system_messages: boolean;
    image_inlining: boolean;
    inline_image_quality: string;
    video_inlining: boolean;
    bypass_status_check: boolean;
    continue_prefill: boolean;
    function_calling: boolean;
    names_behavior: number;
    continue_postfix: string;
    custom_prompt_post_processing: string;
    show_thoughts: boolean;
    reasoning_effort: string;
    enable_web_search: boolean;
    request_images: boolean;
    seed: number;
    n: number;
    bind_preset_to_connection: boolean;
    extensions: CharacterAttachments;
}

export interface BiasPresets {
    "Default (none)": any[];
    "Anti-bond": AntiBond[];
}

export interface AntiBond {
    id: string;
    text: string;
    value: number;
}

export interface PromptOrder {
    character_id: number;
    order: Order[];
}

export interface Order {
    identifier: string;
    enabled: boolean;
}

export interface Prompt {
    name: string;
    system_prompt: boolean;
    role?: string;
    content?: string;
    identifier: string;
    marker?: boolean;
}

export interface PowerUser {
    charListGrid: boolean;
    tokenizer: number;
    token_padding: number;
    collapse_newlines: boolean;
    pin_examples: boolean;
    strip_examples: boolean;
    trim_sentences: boolean;
    always_force_name2: boolean;
    user_prompt_bias: string;
    show_user_prompt_bias: boolean;
    auto_continue: AutoContinue;
    markdown_escape_strings: string;
    chat_truncation: number;
    streaming_fps: number;
    smooth_streaming: boolean;
    smooth_streaming_speed: number;
    fast_ui_mode: boolean;
    avatar_style: number;
    chat_display: number;
    toastr_position: string;
    chat_width: number;
    never_resize_avatars: boolean;
    show_card_avatar_urls: boolean;
    play_message_sound: boolean;
    play_sound_unfocused: boolean;
    auto_save_msg_edits: boolean;
    confirm_message_delete: boolean;
    sort_field: string;
    sort_order: string;
    sort_rule: null;
    font_scale: number;
    blur_strength: number;
    shadow_width: number;
    main_text_color: string;
    italics_text_color: string;
    underline_text_color: string;
    quote_text_color: string;
    blur_tint_color: string;
    chat_tint_color: string;
    user_mes_blur_tint_color: string;
    bot_mes_blur_tint_color: string;
    shadow_color: string;
    border_color: string;
    custom_css: string;
    waifuMode: boolean;
    movingUI: boolean;
    movingUIState: CharacterAttachments;
    movingUIPreset: string;
    noShadows: boolean;
    theme: string;
    gestures: boolean;
    auto_swipe: boolean;
    auto_swipe_minimum_length: number;
    auto_swipe_blacklist: any[];
    auto_swipe_blacklist_threshold: number;
    auto_scroll_chat_to_bottom: boolean;
    auto_fix_generated_markdown: boolean;
    send_on_enter: number;
    console_log_prompts: boolean;
    request_token_probabilities: boolean;
    show_group_chat_queue: boolean;
    allow_name1_display: boolean;
    allow_name2_display: boolean;
    hotswap_enabled: boolean;
    timer_enabled: boolean;
    timestamps_enabled: boolean;
    timestamp_model_icon: boolean;
    mesIDDisplay_enabled: boolean;
    hideChatAvatars_enabled: boolean;
    max_context_unlocked: boolean;
    message_token_count_enabled: boolean;
    expand_message_actions: boolean;
    enableZenSliders: boolean;
    enableLabMode: boolean;
    prefer_character_prompt: boolean;
    prefer_character_jailbreak: boolean;
    quick_continue: boolean;
    quick_impersonate: boolean;
    continue_on_send: boolean;
    trim_spaces: boolean;
    relaxed_api_urls: boolean;
    world_import_dialog: boolean;
    enable_auto_select_input: boolean;
    enable_md_hotkeys: boolean;
    tag_import_setting: number;
    disable_group_trimming: boolean;
    single_line: boolean;
    instruct: Instruct;
    context: Context;
    instruct_derived: boolean;
    context_derived: boolean;
    context_size_derived: boolean;
    model_templates_mappings: CharacterAttachments;
    chat_template_hash: string;
    sysprompt: Sysprompt;
    reasoning: Reasoning;
    personas: Personas;
    default_persona: null;
    persona_descriptions: PersonaDescriptions;
    persona_description: string;
    persona_description_position: number;
    persona_description_role: number;
    persona_description_depth: number;
    persona_description_lorebook: string;
    persona_show_notifications: boolean;
    persona_sort_order: string;
    custom_stopping_strings: string;
    custom_stopping_strings_macro: boolean;
    fuzzy_search: boolean;
    encode_tags: boolean;
    servers: Server[];
    bogus_folders: boolean;
    zoomed_avatar_magnification: boolean;
    show_tag_filters: boolean;
    aux_field: string;
    stscript: Stscript;
    restore_user_input: boolean;
    reduced_motion: boolean;
    compact_input_area: boolean;
    show_swipe_num_all_messages: boolean;
    auto_connect: boolean;
    auto_load_chat: boolean;
    forbid_external_media: boolean;
    external_media_allowed_overrides: any[];
    external_media_forbidden_overrides: any[];
    pin_styles: boolean;
    click_to_edit: boolean;
    ui_mode: number;
    auto_sort_tags: boolean;
    selectSamplers: SelectSamplers;
}

export interface AutoContinue {
    enabled: boolean;
    allow_chat_completions: boolean;
    target_length: number;
}

export interface Context {
    preset: string;
    story_string: string;
    chat_start: string;
    example_separator: string;
    use_stop_strings: boolean;
    names_as_stop_strings: boolean;
    story_string_position: number;
    story_string_depth: number;
    story_string_role: number;
}

export interface Instruct {
    enabled: boolean;
    preset: string;
    input_sequence: string;
    output_sequence: string;
    last_output_sequence: string;
    system_sequence: string;
    stop_sequence: string;
    wrap: boolean;
    macro: boolean;
    names_behavior: string;
    activation_regex: string;
    first_output_sequence: string;
    skip_examples: boolean;
    output_suffix: string;
    input_suffix: string;
    system_suffix: string;
    user_alignment_message: string;
    system_same_as_user: boolean;
    last_system_sequence: string;
    first_input_sequence: string;
    last_input_sequence: string;
    sequences_as_stop_strings: boolean;
    story_string_prefix: string;
    story_string_suffix: string;
}

export interface PersonaDescriptions {
    "user-default.png": UserDefaultPNG;
}

export interface UserDefaultPNG {
    description: string;
    position: number;
}

export interface Personas {
    "user-default.png": string;
}

export interface Reasoning {
    name: string;
    auto_parse: boolean;
    add_to_prompts: boolean;
    auto_expand: boolean;
    show_hidden: boolean;
    prefix: string;
    suffix: string;
    separator: string;
    max_additions: number;
}

export interface SelectSamplers {
    forceHidden: any[];
    forceShown: any[];
}

export interface Server {
    label: string;
    url: string;
    lastConnection: number;
}

export interface Stscript {
    parser: Parser;
    autocomplete: Autocomplete;
}

export interface Autocomplete {
    state: number;
    autoHide: boolean;
    style: string;
    font: Font;
    width: Width;
    select: number;
}

export interface Font {
    scale: number;
}

export interface Width {
    left: number;
    right: number;
}

export interface Parser {
    flags: Record<string, boolean>;
}

export interface Sysprompt {
    enabled: boolean;
    name: string;
    content: string;
}

export interface Proxy {
    name: string;
    url: string;
    password: string;
}

export type TagMap = Record<string, string[]>;

export interface Tag {
    id: string;
    name: string;
    color: string;
}

export interface TextgenerationwebuiSettings {
    temp: number;
    temperature_last: boolean;
    top_p: number;
    top_k: number;
    top_a: number;
    tfs: number;
    epsilon_cutoff: number;
    eta_cutoff: number;
    typical_p: number;
    min_p: number;
    rep_pen: number;
    rep_pen_range: number;
    rep_pen_decay: number;
    rep_pen_slope: number;
    no_repeat_ngram_size: number;
    penalty_alpha: number;
    num_beams: number;
    length_penalty: number;
    min_length: number;
    encoder_rep_pen: number;
    freq_pen: number;
    presence_pen: number;
    skew: number;
    do_sample: boolean;
    early_stopping: boolean;
    dynatemp: boolean;
    min_temp: number;
    max_temp: number;
    dynatemp_exponent: number;
    smoothing_factor: number;
    smoothing_curve: number;
    dry_allowed_length: number;
    dry_multiplier: number;
    dry_base: number;
    dry_sequence_breakers: string;
    dry_penalty_last_n: number;
    max_tokens_second: number;
    seed: number;
    preset: string;
    add_bos_token: boolean;
    stopping_strings: any[];
    ban_eos_token: boolean;
    skip_special_tokens: boolean;
    include_reasoning: boolean;
    streaming: boolean;
    mirostat_mode: number;
    mirostat_tau: number;
    mirostat_eta: number;
    guidance_scale: number;
    negative_prompt: string;
    grammar_string: string;
    json_schema: CharacterAttachments;
    banned_tokens: string;
    global_banned_tokens: string;
    send_banned_tokens: boolean;
    sampler_priority: string[];
    samplers: string[];
    samplers_priorities: string[];
    ignore_eos_token: boolean;
    spaces_between_special_tokens: boolean;
    speculative_ngram: boolean;
    type: string;
    mancer_model: string;
    togetherai_model: string;
    infermaticai_model: string;
    ollama_model: string;
    openrouter_model: string;
    openrouter_providers: any[];
    vllm_model: string;
    aphrodite_model: string;
    dreamgen_model: string;
    tabby_model: string;
    sampler_order: number[];
    logit_bias: any[];
    n: number;
    server_urls: ServerUrls;
    custom_model: string;
    bypass_status_check: boolean;
    openrouter_allow_fallbacks: boolean;
    xtc_threshold: number;
    xtc_probability: number;
    nsigma: number;
    min_keep: number;
    featherless_model: string;
    generic_model: string;
    extensions: CharacterAttachments;
    rep_pen_size: number;
}

export interface ServerUrls {
    koboldcpp: string;
}

export interface WorldInfoSettings {
    world_info: WorldInfo;
    world_info_depth: number;
    world_info_min_activations: number;
    world_info_min_activations_depth_max: number;
    world_info_budget: number;
    world_info_include_names: boolean;
    world_info_recursive: boolean;
    world_info_overflow_alert: boolean;
    world_info_case_sensitive: boolean;
    world_info_match_whole_words: boolean;
    world_info_character_strategy: number;
    world_info_budget_cap: number;
    world_info_use_group_scoring: boolean;
    world_info_max_recursion_steps: number;
}

export interface WorldInfo {
    globalSelect: any[];
}
