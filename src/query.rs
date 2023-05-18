use std::error::Error;

use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "queries/schema.graphql",
    query_path = "queries/component.graphql",
    response_derives = "Debug"
)]
pub struct ComponentQuery;

pub async fn components(
    address: String,
    variables: component_query::Variables,
) -> Result<component_query::ResponseData, Box<dyn Error>> {
    let request_body = ComponentQuery::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{address}/graphql"))
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<component_query::ResponseData> = res.json().await?;

    // let mut file = std::fs::File::create("/home/plertrood/dbg.txt").unwrap();
    // std::io::Write::write_all(&mut file, format!("{:#?}", response_body).as_bytes()).unwrap();

    Ok(response_body.data.unwrap())
}
