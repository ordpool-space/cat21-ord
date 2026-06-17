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
#[test]
fn cat21_json_api_returns_weight_size_and_minted_by() {
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
  assert!(json.weight > 0, "weight should be > 0");
  assert!(json.size > 0, "size should be > 0");
  assert!(json.minted_by.is_some(), "minted_by should be set");
}

#[test]
fn cat21_inscription_page_shows_minted_by_and_current_owner() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21", "--index-addresses"]);

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

  // Should show "minted by" and "current owner" (renamed from "address")
  ord.assert_response_regex(
    format!("/inscription/{inscription_id}"),
    r".*<dt>minted by</dt>.*<dt>current owner</dt>.*",
  );
}

#[test]
fn cat21_inscription_page_shows_fee_rate_weight_and_size() {
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

  ord.assert_response_regex(
    format!("/inscription/{inscription_id}"),
    r".*<dt>fee rate</dt>.*sat/vB.*<dt>weight</dt>.*WU.*<dt>size</dt>.*bytes.*",
  );
}

#[test]
fn cat21_inscriptions_block_json_includes_cat_numbers() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21"]);

  // Two cats in the same block
  core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  let response = ord.json_request("/inscriptions/block/2");
  assert_eq!(response.status(), StatusCode::OK);

  let body = response.text().unwrap();
  // Should contain cat_numbers array with cat #0
  assert!(
    body.contains("\"cat_numbers\":[0]"),
    "block inscriptions should include cat_numbers, got: {body}"
  );
}

#[test]
fn cat21_address_json_includes_cat_numbers() {
  let core = mockcore::spawn();
  core.mine_blocks(1);

  let ord = TestServer::spawn_with_args(
    &core,
    &["--index-cat21", "--index-sats", "--index-addresses"],
  );

  core.broadcast_tx(TransactionTemplate {
    inputs: &[(1, 0, 0, Witness::new())],
    lock_time: 21,
    ..default()
  });

  core.mine_blocks(1);

  // Get the address from the cat's JSON
  let response = ord.json_request("/inscriptions/block/2");
  let inscriptions: api::Inscriptions = serde_json::from_str(&response.text().unwrap()).unwrap();
  let cat_id = &inscriptions.ids[0];

  let cat_json: api::Inscription = serde_json::from_str(
    &ord
      .json_request(format!("/inscription/{cat_id}"))
      .text()
      .unwrap(),
  )
  .unwrap();

  if let Some(address) = cat_json.address {
    let response = ord.json_request(format!("/address/{address}"));
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().unwrap();
    assert!(
      body.contains("\"cat_numbers\""),
      "address JSON should include cat_numbers, got: {body}"
    );
  }
}

#[test]
fn cat21_wallet_in_cat_mode_reads_cat_server() {
  let core = mockcore::spawn();

  // A cat21-ord server renames inscription→cat in every response body.
  let ord = TestServer::spawn_with_args(&core, &["--index-cat21", "--index-sats"]);

  create_wallet(&core, &ord);

  core.mine_blocks(1);

  // `wallet balance` builds the wallet, which GETs /outputs, /inscriptions and
  // /status from the cat server. Without --index-cat21 the wallet chokes on
  // /status (the field blessed_inscriptions was renamed to blessed_cats). With
  // --index-cat21 the wallet un-cats every response first, so ord's canonical
  // api::* structs deserialise and the command succeeds.
  let balance = CommandBuilder::new("--index-cat21 wallet balance")
    .core(&core)
    .ord(&ord)
    .run_and_deserialize_output::<Balance>();

  assert_eq!(balance.cardinal, 50 * COIN_VALUE);
}

#[test]
fn wallet_without_cat21_flag_cannot_parse_cat_server_responses() {
  let core = mockcore::spawn();

  let ord = TestServer::spawn_with_args(&core, &["--index-cat21", "--index-sats"]);

  create_wallet(&core, &ord);

  core.mine_blocks(1);

  // Same cat server, but this command omits --index-cat21, so it does NOT un-cat
  // responses. Building the wallet GETs /status, whose blessed_inscriptions field
  // the server renamed to blessed_cats, and serde fails. This is exactly the
  // failure cat21_decat_json fixes; the test above runs the identical command
  // with the flag and succeeds.
  CommandBuilder::new("wallet balance")
    .core(&core)
    .ord(&ord)
    .expected_exit_code(1)
    .stderr_regex(".*blessed_inscriptions.*")
    .run_and_extract_stdout();
}
// CAT-21 😺 - END
