

use serde::{Deserialize, Serialize};

pub mod api_connector;

pub struct Endpoint {}
impl Endpoint {
    const DEFAULT_CONFIG_LOCATION: &str = "/Users/florian.juesten/.config/trustllo/config.json";
    pub const CARDS: &str = "/cards";
    pub const BOARDS: &str = "/boards";
    pub const LISTS: &str = "/lists";
    pub const MEMBERS: &str = "/members";
    pub const ACTIONS: &str = "/actions";
    pub const SEARCH: &str = "/search";
    pub const CHECKLISTS: &str = "/checklists";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub name: String,
    pub desc: String,
    // descData: null,
    pub closed: bool,
    // dateClosed: null,
    // idOrganization: String,
    // idEnterprise: null,
    // limits: null,
    // pinned: null,
    // shortLink: String,
    // powerUps: any[],
    // dateLastActivity: String,
    // // idTags: any[],
    // datePluginDisable: null,
    // creationMethod: null,
    // ixUpdate: null,
    // enterpriseOwned: bool,
    // idBoardSource: null,
    // idMemberCreator: String,
    pub id: String,
    // starred: bool,
    pub url: String,
    // prefs: Prefs,
    pub subscribed: bool,
    // labelNames: LabelNames,
    // dateLastView: String,
    // shortUrl: String,
    // templateGallery: null,
    // premiumFeatures: String[],
    // memberships: Membership[],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub id: String,
    pub name: String,
    pub closed: bool,
    // pos: uint,
    // softLimit: null,
    pub idBoard: String,
    pub subscribed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    id: String,
    // checkItemStates: null,
    closed: bool,
    // dateLastActivity: String,
    desc: String,
    // descData: null,
    // dueReminder: null,
    idBoard: String,
    idList: String,
    // idMembersVoted: any[],
    // idShort: number,
    // idAttachmentCover: null,
    // idLabels: any[],
    manualCoverAttachment: bool,
    name: String,
    // pos: number,
    shortLink: String,
    // isTemplate: bool,
    // cardRole: null,
    // badges: Badges,
    dueComplete: bool,
    // due?: any,
    // idChecklists: any[],
    // idMembers: any[],
    // labels: any[],
    shortUrl: String,
    // start: null,
    subscribed: bool,
    url: String,
    // cover: Cover,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {}
#[derive(Debug, Serialize, Deserialize)]
pub struct Checklist {}
