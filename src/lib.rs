pub mod shader {
    pub struct Shader {
        vertex: String,
        fragment: String,
    }

    impl Shader {
        pub fn new(vertex: &str, fragment: &str) -> Shader {
            Shader {
                vertex: vertex.to_string(),
                fragment: fragment.to_string(),
            }
        }
        pub fn load_from_file(vertex: &str, fragment: &str) -> Shader {
            Shader {
                vertex: std::fs::read_to_string(vertex).unwrap(),
                fragment: std::fs::read_to_string(fragment).unwrap(),
            }
        }
        pub fn get_vertex(&self) -> &str {
            &self.vertex
        }
        pub fn get_fragment(&self) -> &str {
            &self.fragment
        }
    }
}
