// CAT-21 😺 - START
use super::*;

pub(crate) fn run(settings: Settings) -> SubcommandResult {
  let index = Index::open(&settings)?;
  index.rebuild_home_inscriptions()?;
  Ok(None)
}
// CAT-21 😺 - END
