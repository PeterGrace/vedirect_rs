#[cfg(test)]
use vedirect_rs::enums::ExtractError;
use vedirect_rs::extract_blocks;
use vedirect_rs::ve_direct_parsing::block_to_vedirect;
use vedirect_rs::structs::VEDirectBlock;

const SINGLE_BLOCK: &'static str = "DQpQSUQJMHhBMDU1DQpGVwkxNjQNClNFUiMJSFEyMjMyR1c0M0UNClYJMTMyMzANCkkJLTg3MA0KVlBWCTE0MjANClBQVgkwDQpDUwkwDQpNUFBUCTANCk9SCTB4MDAwMDAwMDENCkVSUgkwDQpMT0FECU9ODQpJTAk4MDANCkgxOQkxNDEwDQpIMjAJMzENCkgyMQk5NQ0KSDIyCTI5DQpIMjMJOTINCkhTRFMJNDcNCkNoZWNrc3VtCUA=";
const STREAM_TEST_DATA: &'static str = "AD8NClBJRAkweEEwNTUNCkZXCTE2NA0KU0VSIwlIUTIyMzJHVzQzRQ0KVgkxMzIzMA0KSQktODcwDQpWUFYJMTQyMA0KUFBWCTANCkNTCTANCk1QUFQJMA0KT1IJMHgwMDAwMDAwMQ0KRVJSCTANCkxPQUQJT04NCklMCTgwMA0KSDE5CTE0MTANCkgyMAkzMQ0KSDIxCTk1DQpIMjIJMjkNCkgyMwk5Mg0KSFNEUwk0Nw0KQ2hlY2tzdW0JQA0KUElECTB4QTA1NQ0KRlcJMTY0DQpTRVIjCUhRMjIzMkdXNDNFDQpWCTEzMjMwDQpJCS04NzANClZQVgkxNDIwDQpQUFYJMA0KQ1MJMA0KTVBQVAkwDQpPUgkweDAwMDAwMDAxDQpFUlIJMA0KTE9BRAlPTg0KSUwJOTAwDQpIMTkJMTQxMA0KSDIwCTMxDQpIMjEJOTUNCkgyMgkyOQ0KSDIzCTkyDQpIU0RTCTQ3DQpDaGVja3N1bQk/DQpQSUQJMHhBMDU1DQpGVwkxNjQNClNFUiMJSFEyMjMyR1c0M0UNClYJMTMyMzANCkkJLTg4MA0KVlBWCTE0MjANClBQVgkwDQpDUwkwDQpNUFBUCTANCk9SCTB4MDAwMDAwMDENCkVSUgkwDQpMT0FECU9ODQpJTAk4MDANCkgxOQkxNDEwDQpIMjAJMzENCkgyMQk5NQ0KSDIyCTI5DQpIMjMJOTINCkhTRFMJNDcNCkNoZWNrc3VtCT8NClBJRAkweEEwNTUNCkZXCTE2NA0KU0VSIwlIUTIyMzJHVzQzRQ0KVgkxMzIzMA0KSQktODgwDQpWUFYJMTM5MA0KUFBWCTANCkNTCTANCk1QUFQJMA0KT1IJMHgwMDAwMDAwMQ0KRVJSCTANCkxPQUQJT04NCklMCTgwMA0KSDE5CTE0MTANCkgyMAkzMQ0KSDIxCTk1DQpIMjIJMjkNCkgyMwk5Mg0KSFNEUwk0Nw0KQ2hlY2tzdW0JOQ0KUElECTB4QTA1NQ0KRlcJMTY0DQpTRVIjCUhRMjIzMkdXNDNFDQpWCTEzMjMwDQpJCS04NjANClZQVgkxMzkwDQpQUFYJMA0KQ1MJMA0KTVBQVAkwDQpPUgkweDAwMDAwMDAxDQpFUlIJMA0KTE9BRAlPTg0KSUwJODAwDQpIMTkJMTQxMA0KSDIwCTMxDQpIMjEJOTUNCkgyMgkyOQ0KSDIzCTkyDQpIU0RTCTQ3DQpDaGVja3N1bQk7DQpQSUQJMHhBMDU1DQpGVwkxNjQNClNFUiMJSFEyMjMyR1c0M0UNClYJMTMyMzANCkkJLTg2MA0KVlBWCTEzOTANClBQVgkwDQpDUwkwDQpNUFBUCTANCk9SCTB4MDAwMDAwMDENCkVSUgkwDQpMT0FECU9ODQpJTAk4MDANCkgxOQkxNDEwDQpIMjAJMzENCkgyMQk5NQ0KSDIyCTI5DQpIMjMJOTINCkhTRFMJNDcNCkNoZWNrc3VtCTsNClBJRAkweEEwNTUNCkZXCTE2NA0KU0VSIwlIUTIyMzJHVzQzRQ0KVgkxMzIzMA0KSQktODgwDQpWUFYJMTM5MA0KUFBWCTANCkNTCTANCk1QUFQJMA0KT1IJMHgwMDAwMDAwMQ0KRVJSCTANCkxPQUQJT04NCklMCTgwMA0KSDE5CTE0MTANCkgyMAkzMQ0KSDIxCTk1DQpIMjIJMjkNCkgyMwk5Mg0KSFNEUwk0Nw0KQ2hlY2tzdW0JOQ0KUElECTB4QTA1NQ0KRlcJMTY0DQpTRVIjCUhRMjIzMkdXNDNFDQpWCTEzMjMwDQpJCS04NzANClZQVgkxMzkwDQpQUFYJMA0KQ1MJMA0KTVBQVAkwDQpPUgkweDAwMDAwMDAxDQpFUlIJMA0KTE9BRAlPTg0KSUwJODAwDQpIMTkJMTQxMA0KSDIwCTMxDQpIMjEJOTUNCkgyMgkyOQ0KSDIzCTkyDQpIU0RTCTQ3DQpDaGVja3N1bQk6DQpQSUQJMHhBMDU1DQpGVwkxNjQNClNFUiMJSFEyMjMyR1c0M0UNClYJMTMyMzANCkkJLTg3MA0KVlBWCTEzOTANClBQVgkwDQpDUwkwDQpNUFBUCTANCk9SCTB4MDAwMDAwMDENCkVSUgkwDQpMT0FECU9ODQpJTAk4MDANCkgxOQkxNDEwDQpIMjAJMzENCkgyMQk5NQ0KSDIyCTI5DQpIMjMJOTINCkhTRFMJNDcNCkNoZWNrc3VtCToNClBJRAkweEEwNTUNCkZXCTE2NA0KU0VSIwlIUTIyMzJHVzQzRQ0KVgkxMzIzMA0KSQktODIwDQpWUFYJMTM5MA0KUFBWCTANCkNTCTANCk1QUFQJMA0KT1IJMHgwMDAwMDAwMQ0KRVJSCTANCkxPQUQJT04NCklMCTgwMA0KSDE5CTE0MTANCkgyMAkzMQ0KSDIxCTk1DQpIMjIJMjkNCkgyMwk5Mg0KSFNEUwk0Nw0KQ2hlY2tzdW0JPw0KUElECTB4QTA1NQ0KRlcJMTY0DQpTRVIjCUhRMjIzMkdXNDNFDQpWCTEzMjMwDQpJCS03OTANClZQVgkxMzkwDQpQUFYJMA0KQ1MJMA0KTVBQVAkwDQpPUgkweDAwMDAwMDAxDQpFUlIJMA0KTE9BRAlPTg0KSUwJODAwDQpIMTkJMTQxMA0KSDIwCTMxDQpIMjEJOTUNCkgyMgkyOQ0KSDIzCTkyDQpIU0RTCTQ3DQpDaGVja3N1bQk5DQpQSUQJMHhBMDU1DQpGVwkxNjQNClNFUiMJSFEyMjMyR1c0M0UNClYJMTMyMzANCkkJLTg3MA0KVlBWCTEzOTANClBQVgkwDQpDUwkwDQpNUFBUCTANCk9SCTB4MDAwMDAwMDENCkVSUgkwDQpMT0FECU9ODQpJTAk4MDANCkgxOQkxNDEwDQpIMjAJMzENCkgyMQk5NQ0KSDIyCTI5DQpIMjMJOTINCkhTRFMJNDcNCkNoZWNrc3VtCTo=";

#[test]
fn get_data_from_block_single_block() {
    let bytes = simple_base64::decode(SINGLE_BLOCK).unwrap();
    let blocks = extract_blocks(&bytes).unwrap();
    assert_eq!(blocks.len(), 1);
    let veblock = block_to_vedirect(&blocks[0]);
    assert_eq!(veblock.maxpower_today,95_i32);
}
#[test]
fn get_data_from_block_stream() {
    let bytes = simple_base64::decode(STREAM_TEST_DATA).unwrap();
    let blocks = extract_blocks(&bytes).unwrap();
    assert_eq!(blocks.len(), 12);
    for block in blocks.iter() {
        let data = block_to_vedirect(block);
        println!("{data:#?}");
    }
    let veblock = block_to_vedirect(&blocks[0]);
    println!("{veblock:#?}");
    //assert_eq!(veblock.maxpower_today,1i32);
}