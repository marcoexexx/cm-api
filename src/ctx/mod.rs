#[derive(Clone)]
pub struct Ctx {
  pub user_id: u64,
}

impl Ctx {
  pub fn new(user_id: u64) -> Ctx {
    Ctx { user_id }
  }
}

impl Ctx {
  pub fn user_id(&self) -> u64 {
    self.user_id
  }
}
