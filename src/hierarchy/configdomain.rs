pub enum ConfigDomain {
    None = 0b00000000,
    
    EngineBase = 0b00000010,
    Engine = 0b00000100,
    Project = 0b00001000,
    ProjectGenerated = 0b00010000,
}