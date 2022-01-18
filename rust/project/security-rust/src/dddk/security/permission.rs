#[derive(Eq, Hash)]
pub struct Permission {
    permission: String,
}

impl Permission {
    pub fn new(permission: String) -> Permission {
        Permission {
            permission
        }
    }

    pub fn as_string(&self) -> String {
        self.permission.clone()
    }
}

impl PartialEq<Self> for Permission {
    fn eq(&self, other: &Self) -> bool {
        self.as_string() == other.as_string()
    }
}

impl Clone for Permission {
    fn clone(&self) -> Self {
        Permission::new(self.as_string())
    }
}
