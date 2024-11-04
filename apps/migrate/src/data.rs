use db::{users, DbPool, NewUser, User};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use itertools::Itertools;
use octocrab::Octocrab;

use crate::{package, pkg::LegacyPackage, util::AsyncFilterMap};

pub fn get_packages() -> Vec<LegacyPackage> {
    let mut pkgs = Vec::new();

    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "kjspkg-compat-layer"));
    pkgs.push(package!("Gcat101"; + "brainfuck-interpreter"; = "gcatkjspkgs"; "brainfuckJS"));
    pkgs.push(package!("Gcat101"; + "chunk-randomizer"; = "gcatkjspkgs"; "ChunkRandomizer"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "trippers-titles"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "thermalootjs"));
    pkgs.push(package!("Gcat101"; + "amogus"; = "gcatkjspkgs"; "kjspkg-test"));

    pkgs.push(
        package!("Gcat101"; + "more-recipe-types"; = "gcatkjspkgs"; "kubejs-more-recipe-types"),
    );

    pkgs.push(package!("Gcat101"; + "boss-tools-oxygen"; = "gcatkjspkgs"; "boss-tools-oxygen-js"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "lightlevel-spawning-backport"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "bad-apple"));
    pkgs.push(package!("glomdom"; "unifico"; @ "master"));
    pkgs.push(package!("structurejs"; = "Qualia765"; "structureJSv2"));
    pkgs.push(package!("cutscenejs"; = "Qualia765"; "cutscenes-kubejs"));
    pkgs.push(package!("climate-compass"; = "Qualia765"; "climate-kubejs"));
    pkgs.push(package!("EvanHsieh0415"; + "boss-death-world-notification"; = "Mango-Minecraft-Project"; "Mango-KubeJScripts"; "scripts/World_Notification"));
    pkgs.push(package!("EvanHsieh0415"; + "create-delight"; = "Mango-Minecraft-Project"; "Mango-KubeJScripts"; "scripts/Create_Delight"));
    pkgs.push(package!("gregicsifting"; = "Drackion"; "GregicSifting"));
    pkgs.push(package!("cauldron-hand-washing"; = "V972"; "CauldronHandWashing"));

    pkgs.push(
        package!("buddingamethyst"; = "Liopyu"; "Lios-KubeJS-Scripts"; "scripts/buddingamethyst"),
    );

    pkgs.push(package!("gravescroll"; = "Liopyu"; "Lios-KubeJS-Scripts"; "scripts/gravescroll"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "entity-snip-snip"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "not-neat"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "command-restrictor"; @ "1.18"));
    pkgs.push(package!("Gcat101"; + "command-restrictor-6"; = "gcatkjspkgs"; "command-restrictor"; @ "1.19+"));
    pkgs.push(package!("TheonlyTazz"; + "craft-js"; = "breakinblocks"; "Craft.js"; @ "1.18"));
    pkgs.push(package!("TheonlyTazz"; + "craft-js-6"; = "breakinblocks"; "Craft.js"; @ "1.19+"));
    pkgs.push(package!("Gcat101"; + "soljs"; = "gcatkjspkgs"; "SolJS"));
    pkgs.push(package!("Gcat101"; + "pongjs"; = "gcatkjspkgs"; "PongJS"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "create-manual"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "torch-by-pickaxe"));
    pkgs.push(package!("update-notifier"; = "KostromDan"; "Update-Notifier"));

    pkgs.push(
        package!("Gcat101"; + "leonariz-utils"; = "gcatkjspkgs"; "leonarizs-pkgs"; @ "utilities"),
    );

    pkgs.push(
        package!("Gcat101"; + "curios-registry"; = "gcatkjspkgs"; "leonarizs-pkgs"; @ "curios"),
    );

    pkgs.push(
        package!("Gcat101"; + "replant-wheat-seeds"; = "gcatkjspkgs"; "leonarizs-pkgs"; @ "wheat"),
    );

    pkgs.push(package!("squoshi"; "playeranimator-kubejs"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "entropy-manipulator-create-depot"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "hardcore-air-supply"));
    pkgs.push(package!("Gcat101"; + "lodestone-js"; = "gcatkjspkgs"; "lodestone-library"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "lodestone-screenshake"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "create-external-sequenced-assembly"));
    pkgs.push(package!("Rimevel"; + "create-addition-delight-compat"; = "Rimevel"; "createaddition_delight_compat"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "create-depot-crafting"; @ "fabric"));
    pkgs.push(package!("Gcat101"; + "create-depot-crafting-forge"; = "gcatkjspkgs"; "create-depot-crafting"; @ "forge"));
    pkgs.push(package!("Gcat101"; + "tic-deco-fix"; = "gcatkjspkgs"; "tcondecofix"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "create-water-bucket-mixing-fix"));
    pkgs.push(package!("Gcat101"; + "chipped-rei-group"; = "gcatkjspkgs"; "group-chipped-rei"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "mekanism-custom-infused-type"; @ "1.18"));
    pkgs.push(package!("Gcat101"; + "gcatkjspkgs"; "extended-days"));

    pkgs.push(
        package!("grindstonerepairjs"; = "beckadamtheinventor"; "GrindstoneRepairJS"; @ "master"),
    );

    pkgs.push(package!("kjs-datagen-scripts"; = "FooterManDev"; "KJS-Datagen-Scripts"));
    pkgs.push(package!("kubejs-lootbags"; = "Kisuny"; "KubeJS-lootbags"));
    pkgs.push(package!("kubejs-copypaste"; = "DevDyna"; "Kubejs-CopyPaste"));
    pkgs.push(package!("script-collection"; = "DevDyna"; "kubejs-testing"));
    pkgs.push(package!("parse-text-format"; = "omgimanerd"; "kubejs-utilities"; @ "master"; "parse-text-format"));
    pkgs.push(package!("sequenced-assembly"; = "omgimanerd"; "kubejs-utilities"; @ "master"; "SequencedAssembly"));
    pkgs.push(package!("faster-climbing"; = "FalAut"; "FalAut-KubeJScripts"; @ "2001"; "scripts/Faster_Climbing"));
    pkgs.push(package!("numerical-mana-in-jade"; = "FalAut"; "FalAut-KubeJScripts"; @ "2001"; "scripts/Numerical_Mana_In_Jade"));
    pkgs.push(package!("quick-equipment-swapping"; = "FalAut"; "FalAut-KubeJScripts"; @ "2001"; "scripts/Quick_Equipment_Swapping"));
    pkgs.push(package!("inventory-sort"; = "Notenoughmail"; "kube-scripts"; "1-20/inventory-sort"));

    pkgs.push(
        package!("auto-third-person"; = "Notenoughmail"; "kube-scripts"; "1-20/auto-third-person"),
    );

    pkgs.push(
        package!("lychee-schema"; = "9thCore"; "KubeJS_Customs"; "kubejs-packages/lychee-schema"),
    );

    pkgs
}

pub async fn get_users(pkgs: &Vec<LegacyPackage>, pool: &DbPool) -> Vec<NewUser> {
    pkgs.iter()
        .unique_by(|v| v.author)
        .map(|v| *v)
        .collect::<Vec<_>>()
        .filter_map_async(pool, |pool, v| async move {
            if let Some(user) = users::table
                .filter(users::username.eq(v.author.to_string()))
                .select(User::as_select())
                .first(&mut pool.get().await.unwrap())
                .await
                .optional()
                .unwrap_or_default()
            {
                info!(
                    "[Users] Found existing user for name '{}' with ID {}!",
                    user.username, user.id
                );
                None
            } else {
                info!("[Users] No existing user found for name '{}'!", v.author);
                Some(NewUser {
                    github_id: *Octocrab::builder()
                        .build()
                        .unwrap()
                        .users(v.author)
                        .profile()
                        .await
                        .unwrap()
                        .id as i32,
                    username: v.author.into(),
                })
            }
        })
        .await
}

pub fn get_user_names(pkgs: &Vec<LegacyPackage>) -> Vec<&'static str> {
    pkgs.iter()
        .unique_by(|v| v.author)
        .map(|v| v.author)
        .collect()
}
