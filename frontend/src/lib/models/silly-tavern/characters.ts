export interface Characters {
    name: string;
    description: string;
    personality: string;
    scenario: string;
    first_mes: string;
    mes_example: string;
    creatorcomment: Record<string, string>;
    avatar: string;
    chat: string;
    talkativeness: string;
    fav: boolean;
    tags: string[];
    spec: Spec;
    spec_version: string;
    data: Data;
    create_date: string;
    json_data: string;
    date_added: number;
    chat_size: number;
    date_last_chat: number;
    data_size: number;
}

export interface Data {
    name: string;
    description: string;
    personality: string;
    scenario: string;
    first_mes: string;
    mes_example: string;
    creator_notes: Record<string, string>;
    system_prompt: string;
    post_history_instructions: string;
    tags: string[];
    creator: CreatorEnum;
    character_version: CharacterVersion;
    alternate_greetings: string[];
    extensions: DataExtensions;
    group_only_greetings: string[];
    character_book?: CharacterBook;
    avatar?: string;
}

export interface CharacterBook {
    entries: Entry[];
    name: string;
}

export interface Entry {
    id: number;
    keys: string[];
    secondary_keys: string[];
    comment: string;
    content: string;
    constant: boolean;
    selective: boolean;
    insertion_order: number;
    enabled: boolean;
    position: Position;
    use_regex: boolean;
    extensions: EntryExtensions;
}

export interface EntryExtensions {
    position: number;
    exclude_recursion: boolean;
    display_index: number;
    probability: number;
    useProbability: boolean;
    depth: number;
    selectiveLogic: number;
    group: string;
    group_override: boolean;
    group_weight: number;
    prevent_recursion: boolean;
    delay_until_recursion: boolean;
    scan_depth: null;
    match_whole_words: null;
    use_group_scoring: boolean;
    case_sensitive: null;
    automation_id: string;
    role: number;
    vectorized: boolean;
    sticky: number;
    cooldown: number;
    delay: number;
    match_persona_description?: boolean;
    match_character_description?: boolean;
    match_character_personality?: boolean;
    match_character_depth_prompt?: boolean;
    match_scenario?: boolean;
    match_creator_notes?: boolean;
    triggers?: string[];
    ignore_budget?: boolean;
}

export enum Position {
    AfterChar = "after_char",
    BeforeChar = "before_char",
}

export enum CharacterVersion {
    Empty = "",
    The100 = "1.0.0",
}

export enum CreatorEnum {
    Empty = "",
    OtisAlejandro = "OtisAlejandro",
}

export interface DataExtensions {
    talkativeness: string;
    fav: boolean;
    world: string;
    depth_prompt: DepthPrompt;
    sd_character_prompt?: SDCharacterPrompt;
}

export interface DepthPrompt {
    prompt: string;
    depth: number;
    role: Role;
}

export enum Role {
    System = "system",
}

export interface SDCharacterPrompt {
    positive: string;
    negative: string;
}

export enum Spec {
    CharaCardV3 = "chara_card_v3",
}
