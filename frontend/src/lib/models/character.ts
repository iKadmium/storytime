export interface Voice {
    temperature: number;
    exaggeration: number;
    cfgWeight: number;
    speedFactor: number;
    voiceName: string;
}

export interface Character {
    name: string;
    description: string;
    personality: string;
    background: string;
    voice?: Voice;
}

export interface CreateCharacterRequest {
    name: string;
    description: string;
    personality: string;
    background: string;
    voice?: Voice;
}

export interface UpdateCharacterRequest {
    description?: string;
    personality?: string;
    background?: string;
    voice?: Voice;
}

export interface ApiResponse<T> {
    success: boolean;
    data?: T;
    message: string;
}

export type CharacterListResponse = ApiResponse<Character[]>;
export type CharacterResponse = ApiResponse<Character>;
export type CharacterDeleteResponse = ApiResponse<null>;