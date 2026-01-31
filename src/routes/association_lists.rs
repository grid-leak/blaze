use crate::{
    models::association_lists::{AssociationList, GetListsResponse},
    packet::Packet,
    session::SessionLink,
};

static ASSOCIATION_LISTS: &[AssociationList] = &[
    AssociationList::friend_list(),
    AssociationList::follow_list(),
    AssociationList::block_list(),
];

pub async fn get_lists(_: &SessionLink, packet: &Packet) -> Packet {
    Packet::reply(
        packet,
        GetListsResponse {
            list: ASSOCIATION_LISTS,
        },
    )
}
