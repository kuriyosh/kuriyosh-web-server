use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(description = "他人に読まれることを想定して書いているもの")]
pub struct Blog {
    id: String,
    thumbnail_url: String,
    content: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "特定のトピックについての体系的なメモ")]
pub struct Note {
    id: String,
    name: String,
    icon_url: String,
    content: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "雑多なトピックに関する自分用のメモ")]
pub struct Memo {
    id: String,
    name: String,
    tags: Vec<String>,
    content: String,
}
