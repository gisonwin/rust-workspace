use std::error::Error;
use std::fmt::format;
use scraper::Element;
use spider::tokio;
use spider::website::Website;
use tokio::io::AsyncWriteExt;

struct Country {
    name: String, // 国家名称
    capital: String, //首都
    population: String, //人口数量
    area: String, //国土面积
}


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let url = "https://www.scrapethissite.com/pages/simple/";
//     let resp = reqwest::get(url).await?.text().await?;
//     // let html = resp.text()?;
//     // println!("{resp}");
//     let document = scraper::Html::parse_document(&resp);
//     let html_country_info_box_selector = scraper::Selector::parse(".country")?;
//     let html_country_info_box_element = document.select(&html_country_info_box_selector).next().ok_or("Country info box element not found")?;
//
//     let country_name_selector = scraper::Selector::parse(".country-name")?;
//     let name = html_country_info_box_element.select(&country_name_selector).next().map(|e| e.text().collect::<String>().trim().to_owned()).ok_or("Country name box element not found")?;
//     println!("{}", name);
//     let capital_selector = scraper::Selector::parse(".country-capital")?;
//     let capital = html_country_info_box_element.select(&capital_selector).next().map(|e| e.text().collect::<String>().trim().to_owned()).ok_or("country capital not found")?;
//     println!("{}", capital);
//     let population_selector = scraper::Selector::parse(".country-population")?;
//     let population = html_country_info_box_element.select(&population_selector).next().map(|e| e.text().collect::<String>().trim().to_owned()).ok_or("Population box element not found")?;
//     println!("{}", population);
//     let area_selector = scraper::Selector::parse(".country-area")?;
//    let area= html_country_info_box_element.select(&area_selector).next().map(|e|e.text().collect::<String>().trim().to_owned()).ok_or("Area box element not found")?;
//
//
//     Ok(())
//     // let doc = Document::from(resp.as_str());
//     //  for node in doc.find(Name("div").and(Class("hd"))){
//     //       let title = node.find(Name("a")).next().593acf54fe2005c60a6aabca7d9a0c99().text();
//     //       let rating = node.parent().593acf54fe2005c60a6aabca7d9a0c99().find(Class("rating_num")).next().593acf54fe2005c60a6aabca7d9a0c99().text();
//     //        println!("title: {}, rating: {}", title, rating);
//     // }
//     //   Ok(())
//
// }

#[tokio::main]
async fn main() {
    let mut website: Website = Website::new("http://www.sgvindex.com/");
    // website.crawl().await;'
    // let links = website.get_links();
    // links.iter().for_each(|link| { println!("-{:#?}", link.as_ref()) });

    let mut rx2 = website.subscribe(16).unwrap();
    tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Ok(res) = rx2.recv().await {
            let _ = stdout.write_all(format!("- {} \n", res.get_url()).as_bytes()).await;
        }
    });
    website.crawl().await;
}