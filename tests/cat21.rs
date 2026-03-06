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
    ".*<h1>Cat 0</h1>.*",
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
      r".*data-txid={cat_txid}.*data-block-hash=[[:xdigit:]]{{64}}.*data-fee=\d+.*data-weight=\d+.*cat21\.js.*"
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

  // All data attributes should be present and correct
  ord.assert_response_regex(
    format!("/preview/{inscription_id}"),
    format!(
      r".*data-txid={cat_txid}.*data-block-hash=[[:xdigit:]]{{64}}.*data-fee=1234.*data-weight=\d+.*"
    ),
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
  ord.assert_response_regex(format!("/inscription/{id_0}"), r".*<h1>Cat 0</h1>.*");
  ord.assert_response_regex(format!("/inscription/{id_1}"), r".*<h1>Cat 1</h1>.*");
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

  // The page title should say "Cat 0"
  ord.assert_response_regex(
    format!("/inscription/{inscription_id}"),
    r".*<title>Cat 0</title>.*",
  );
}

#[test]
fn cat21_json_api_returns_inscription() {
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

  let response = ord.json_request(format!("/inscription/{inscription_id}"));
  assert_eq!(response.status(), StatusCode::OK);

  let json: api::Inscription = serde_json::from_str(&response.text().unwrap()).unwrap();
  assert_eq!(json.id, inscription_id);
  assert_eq!(json.number, 0);
  assert_eq!(json.content_type, None);
  assert_eq!(json.content_length, None);
}

#[test]
fn cat21_inscriptions_page_shows_cats_heading() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  ord.assert_response_regex("/inscriptions", r".*<h1>All Cats</h1>.*");
}

#[test]
fn cat21_content_returns_404() {
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

  // Cats have no on-chain content — SVG is rendered client-side in preview
  let response = ord.request(format!("/content/{inscription_id}"));
  assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn cat21_inscription_page_has_traits_section() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  let cat_txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    fee: 5000,
    ..default()
  });

  core.mine_blocks(1);

  let inscription_id = InscriptionId {
    txid: cat_txid,
    index: 0,
  };

  // The inscription page should include the traits container with all data attributes
  ord.assert_response_regex(
    format!("/inscription/{inscription_id}"),
    format!(
      r#".*<div id="cat21-traits"\s+data-txid={cat_txid}\s+data-block-hash=[[:xdigit:]]{{64}}\s+data-fee=5000\s+data-weight=\d+>.*cat21-traits\.css.*cat21-traits\.js.*"#
    ),
  );
}
#[test]
fn cat_url_rewrites_to_inscription() {
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

  // /cat/ URL should serve the same content as /inscription/
  ord.assert_response_regex(format!("/cat/{inscription_id}"), ".*<h1>Cat 0</h1>.*");
}

#[test]
fn cats_url_rewrites_to_inscriptions() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  // /cats URL should serve the same content as /inscriptions
  ord.assert_response_regex("/cats", r".*<h1>All Cats</h1>.*");
}

#[test]
fn cat21_json_tx_uses_cat_terminology() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  let cat_txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  let response = ord.json_request(format!("/tx/{cat_txid}"));
  assert_eq!(response.status(), StatusCode::OK);

  let body = response.text().unwrap();
  assert!(
    !body.contains("inscription"),
    "JSON /tx/ response should not contain 'inscription', got: {body}"
  );
  assert!(
    body.contains("cat_count"),
    "JSON /tx/ response should contain 'cat_count', got: {body}"
  );
}

#[test]
fn cat21_json_output_uses_cat_terminology() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  let cat_txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  let response = ord.json_request(format!("/output/{cat_txid}:0"));
  assert_eq!(response.status(), StatusCode::OK);

  let body = response.text().unwrap();
  assert!(
    !body.contains("inscription"),
    "JSON /output/ response should not contain 'inscription', got: {body}"
  );
  assert!(
    body.contains("cats"),
    "JSON /output/ response should contain 'cats', got: {body}"
  );
}

#[test]
fn cat21_json_block_uses_cat_terminology() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  let response = ord.json_request("/block/2");
  assert_eq!(response.status(), StatusCode::OK);

  let body = response.text().unwrap();
  assert!(
    !body.contains("inscription"),
    "JSON /block/ response should not contain 'inscription', got: {body}"
  );
  assert!(
    body.contains("cats"),
    "JSON /block/ response should contain 'cats', got: {body}"
  );
}

#[test]
fn cat21_sat_name_preserved_in_json() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21", "--index-sats"]);

  // Sat named "inscription" (sat #749485600560504)
  let response = ord.json_request("/sat/749485600560504");
  assert_eq!(response.status(), StatusCode::OK);

  let body = response.text().unwrap();
  // The sat name must be "inscription", not "cat"
  assert!(
    body.contains("\"name\":\"inscription\""),
    "sat name should be 'inscription', got: {body}"
  );
  // But the field name should still be renamed
  assert!(
    !body.contains("\"inscriptions\""),
    "field name 'inscriptions' should be renamed to 'cats', got: {body}"
  );
}

#[test]
fn cat21_sat_name_preserved_in_html() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21", "--index-sats"]);

  // Sat named "inscription"
  ord.assert_response_regex(
    "/sat/749485600560504",
    r".*<dt>name</dt><dd>inscription</dd>.*",
  );
}

#[test]
fn two_cat21_transactions_in_same_block() {
  let core = mockcore::spawn();
  core.mine_blocks(2);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  // Two cat txs in the same block (using different UTXOs)
  let cat_txid_0 = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });
  let cat_txid_1 = core.broadcast_tx(TransactionTemplate {
    inputs: &[(2, 0, 0, Witness::new())],
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

  // Both should be indexed with sequential numbers
  ord.assert_response_regex(format!("/inscription/{id_0}"), r".*<h1>Cat 0</h1>.*");
  ord.assert_response_regex(format!("/inscription/{id_1}"), r".*<h1>Cat 1</h1>.*");
}

#[test]
fn cats_paginated_url_rewrites_to_inscriptions() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  // /cats/0 should serve the same content as /inscriptions/0
  ord.assert_response_regex("/cats/0", r".*<h1>All Cats</h1>.*");
}

#[test]
fn index_cat21_and_no_index_inscriptions_are_mutually_exclusive() {
  CommandBuilder::new("--index-cat21 --no-index-inscriptions settings")
    .stderr_regex(".*--index-cat21 and --no-index-inscriptions are mutually exclusive.*")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}

#[test]
fn without_index_cat21_flag_nlocktime21_is_not_indexed() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  // No --index-cat21 flag
  let ord = TestServer::spawn(&core);

  let txid = core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  let inscription_id = InscriptionId { txid, index: 0 };

  // Without --index-cat21, nLockTime=21 should NOT be treated as an inscription
  let response = ord.request(format!("/inscription/{inscription_id}"));
  assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
// CAT-21 😺 - END
