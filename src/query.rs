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
    // println!("{:#?}", response_body);
    Ok(response_body.data.unwrap())
}
