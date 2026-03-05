use super::*;

#[derive(Boilerplate)]
pub(crate) struct HomeHtml {
  pub(crate) index_cat21: bool, // CAT-21 😺
  pub(crate) inscriptions: Vec<InscriptionId>,
}

impl PageContent for HomeHtml {
  fn title(&self) -> String {
    // CAT-21 😺 - START
    if self.index_cat21 {
      "CAT-21".to_string()
    } else {
      "Ordinals".to_string()
    }
    // CAT-21 😺 - END
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn html() {
    assert_regex_match!(
      HomeHtml {
        index_cat21: false, // CAT-21 😺
        inscriptions: vec![inscription_id(1), inscription_id(2)],
      }
      .to_string()
      .unindent(),
      "<h1>Latest Inscriptions</h1>
      <div class=thumbnails>
        <a href=/inscription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
        <a href=/inscription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
      </div>
      "
      .unindent(),
    );
  }
}
