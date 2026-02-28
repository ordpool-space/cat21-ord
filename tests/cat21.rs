// CAT-21 😺 - START
use super::*;

#[test]
fn cat21_transaction_is_indexed_as_inscription() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  // Broadcast a transaction with nLockTime=21
  let cat_txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  let inscription_id = InscriptionId {
    txid: cat_txid,
    index: 0,
  };

  // The cat should appear as inscription #0
  ord.assert_response_regex(
    format!("/inscription/{inscription_id}"),
    format!(".*<h1>Cat Number 0</h1>.*"),
  );
}

#[test]
fn non_cat21_transaction_is_not_indexed() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  // Broadcast a normal transaction (nLockTime=0)
  let normal_txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    ..default()
  });

  core.mine_blocks(1);

  let inscription_id = InscriptionId {
    txid: normal_txid,
    index: 0,
  };

  // Should return 404 — not indexed
  let response = ord.request(format!("/inscription/{inscription_id}"));
  assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn cat21_preview_returns_cat21_template() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  let cat_txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  let inscription_id = InscriptionId {
    txid: cat_txid,
    index: 0,
  };

  // Preview should contain the cat21 template with data attributes
  ord.assert_response_regex(
    format!("/preview/{inscription_id}"),
    format!(
      r".*data-inscription={inscription_id}.*data-block-hash=[[:xdigit:]]{{64}}.*data-fee=\d+.*data-weight=\d+.*cat21\.js.*"
    ),
  );
}

#[test]
fn cat21_preview_has_correct_fee_and_weight() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let cat_txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    fee: 1234,
    ..default()
  });

  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  let inscription_id = InscriptionId {
    txid: cat_txid,
    index: 0,
  };

  // Fee should match what we set
  ord.assert_response_regex(
    format!("/preview/{inscription_id}"),
    format!(r".*data-fee=1234.*data-weight=\d+.*"),
  );
}

#[test]
fn cat_route_redirects_to_inscription() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  let cat_txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  // /cat/<txid> should redirect to /inscription/<txid>i0
  let response = reqwest::blocking::Client::builder()
    .redirect(reqwest::redirect::Policy::none())
    .build()
    .unwrap()
    .get(
      ord
        .url()
        .join(&format!("/cat/{cat_txid}"))
        .unwrap(),
    )
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::SEE_OTHER);
  assert_eq!(
    response.headers().get("location").unwrap().to_str().unwrap(),
    format!("/inscription/{cat_txid}i0")
  );
}

#[test]
fn cats_route_redirects_to_inscriptions() {
  let core = mockcore::spawn();
  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  let response = reqwest::blocking::Client::builder()
    .redirect(reqwest::redirect::Policy::none())
    .build()
    .unwrap()
    .get(ord.url().join("/cats").unwrap())
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::SEE_OTHER);
  assert_eq!(
    response.headers().get("location").unwrap().to_str().unwrap(),
    "/inscriptions"
  );
}

#[test]
fn cats_paginated_route_redirects() {
  let core = mockcore::spawn();
  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  let response = reqwest::blocking::Client::builder()
    .redirect(reqwest::redirect::Policy::none())
    .build()
    .unwrap()
    .get(ord.url().join("/cats/5").unwrap())
    .send()
    .unwrap();

  assert_eq!(response.status(), StatusCode::SEE_OTHER);
  assert_eq!(
    response.headers().get("location").unwrap().to_str().unwrap(),
    "/inscriptions/5"
  );
}

#[test]
fn multiple_cat21_transactions_get_sequential_numbers() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  // First cat
  let cat_txid_0 = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });
  core.mine_blocks(1);

  // Second cat (need another UTXO)
  core.mine_blocks(1);
  let cat_txid_1 = core.broadcast_tx(TransactionTemplate {
    inputs: &[(3, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });
  core.mine_blocks(1);

  let id_0 = InscriptionId {
    txid: cat_txid_0,
    index: 0,
  };
  let id_1 = InscriptionId {
    txid: cat_txid_1,
    index: 0,
  };

  // Cat #0 and Cat #1
  ord.assert_response_regex(
    format!("/inscription/{id_0}"),
    r".*<h1>Cat Number 0</h1>.*",
  );
  ord.assert_response_regex(
    format!("/inscription/{id_1}"),
    r".*<h1>Cat Number 1</h1>.*",
  );
}

#[test]
fn cat21_inscription_page_shows_cat_heading_in_title() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  let cat_txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  let inscription_id = InscriptionId {
    txid: cat_txid,
    index: 0,
  };

  // The page title should say "Cat Number 0"
  ord.assert_response_regex(
    format!("/inscription/{inscription_id}"),
    r".*<title>Cat Number 0</title>.*",
  );
}
// CAT-21 😺 - END
