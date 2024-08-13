use regex::Regex;

//1. UUIDs: Standard UUIDs like fa77c3e6-0514-465b-9962-320643a3ac97.
// 2. Numeric IDs: Numeric values with three or more digits.
// 3. Alphanumeric IDs: Alphanumeric strings with seven or more characters containing letters and digits.
// If the URL( the part of url after the last /) contains a single ID, it should be replaced with  __ID__. If it contains multiple IDs, they should be replaced with __IDs__.

fn fuzzy_replace_ids(url: &str) -> String {
    let uuid_re = Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}").unwrap();
    let numeric_id_re = Regex::new(r"\d{3,}").unwrap();
    let alphanumeric_re = Regex::new(r"[a-zA-Z\d]{7,}").unwrap();

    
    let mut parts: Vec<&str> = url.split('/').collect();
    let mut id_count = 0;

    for part in &mut parts {
        let subparts: Vec<&str> = part.split(|c: char| c == ',' || c == '_').collect();
        let mut is_id = false;

        for subpart in &subparts {
            
            if uuid_re.is_match(subpart) {
                is_id = true;
                id_count += 1;
                // break;
            }

            
            if numeric_id_re.is_match(subpart) {
                is_id = true;
                id_count += 1;
                // break;
            }

        
            if alphanumeric_re.is_match(subpart) && subpart.chars().any(|c| c.is_alphabetic()) && subpart.chars().any(|c| c.is_digit(10)) {
                is_id = true;
                id_count += 1;
                // break;
            }
        }

        if is_id {
            // id_count += 1;
            *part = "__ID__";
        }
    }

    println!("id_count= {}",id_count);


    if id_count > 1 {
        for part in &mut parts {
            if *part == "__ID__" {
                *part = "__IDs__";
            }
        }
    }

    parts.join("/")
}


fn main() {
    let test_cases = vec![
        ("/ping/fa77c3e6-0514-465b-9962-320643a3ac97", "/ping/__ID__"),
        ("/workspaces/ws-1406ef2f-5758-4ebd-8c0e-bf2f9f5a1952/api/v1/status/buildinfo", "/workspaces/__ID__/api/v1/status/buildinfo"),
        ("/exec/0jJLMK1-", "/exec/__ID__"),
        ("/exec/21Bn-4Dr", "/exec/__ID__"),
        ("/exec/1seRxK0t", "/exec/__ID__"),
        ("/product-categories/[33845d68-d5a9-4f19-aac6-47b8f5fde632]", "/product-categories/__IDs__"),
        ("/store_items2/_doc/01RNX0D9XM", "/store_items2/_doc/__ID__"),
        ("/store_items2/_doc/02DGL9W3WA", "/store_items2/_doc/__ID__"),
        ("/upstreams/138d46f8-046c-44dc-8d66-f1d8085c42cf", "/upstreams/__ID__"),
        ("/v1/availability/XFRS39N80W", "/v1/availability/__ID__"),
        ("/v1/skus/01JS1MJP9M,47U1ZXNUWO,LYM6X3NBJX,V3ZB916DYW,CDQ456GJ7M,3GPCUM6CM1,ZD0JR6R20S,Q1BL7MTX6S,1000GJXLNB,K3KQYD85UD,J2Z93H637H,ZU4S4HY27F,04KFFWOP9T,8HNU3AQ3NY,B8GLJ7TORK,AZL63VH2MC", "/v1/skus/__IDs__"),
    ];

    for (input, expected) in test_cases {
        let result = fuzzy_replace_ids(input);
        if result != expected {
            println!("Test failed for URL: {}", input);
            println!("Expected: {}", expected);
            println!("Got:      {}", result);
        } else {
            println!("Test passed for URL: {}", input);
        }
        println!("--------------------------------------------");
    }

    println!("Test run completed!");
}
