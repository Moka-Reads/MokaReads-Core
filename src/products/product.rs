use std::cmp::{max, Ordering};
use crate::prices::{TIER0, TIER1, TIER2, TIER3, TIER4, TIER5};
use serde::{Deserialize, Serialize};
use std::io::{Result, stdin};
use std::path::PathBuf;

const UDEMY_DEF: f64 = 35.99;
// Paperback costs in CAD for premium color ink with 42-828 pages
const PAPERBACK_FIXED: f64 = 1.26;
const PAPERBACK_PER_PAGE: f64 = 0.085;

// Hardcover costs in USD for premium color ink with 75-550 pages
const HARDCOVER_FIXED: f64 = 5.65;
const HARDCOVER_PER_PAGE: f64 = 0.065;

fn paperback_print_cost(pages: u64) -> f64 {
    PAPERBACK_FIXED + (pages as f64 * PAPERBACK_PER_PAGE)
}

fn hardcover_print_cost(pages: u64) -> f64{
    HARDCOVER_FIXED + (pages as f64 * HARDCOVER_PER_PAGE)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    // name of the product
    name: String,
    // product's type
    prod_ty: ProductType,
    // Book prices if Publication
    book_price: Option<BookPrices>,
    // Price if Udemy course
    udemy_price: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookPrices {
    // price from MoKa Reads
    ebook: f64,
    // Amazon.ca Kindle price
    kindle: f64,
    // Amazon.ca Paperback price
    paperback: f64,
    // Amazon.com Hardcover price
    hardcover: f64,
}

impl BookPrices {
    fn paperback_royalty(&self, pages: u64) -> f64{
        self.paperback * 0.6 - paperback_print_cost(pages)
    }
    /// This price is under USD
    fn hardcover_royalty(&self, pages: u64) -> f64{
        self.hardcover * 0.6 - hardcover_print_cost(pages)
    }

    fn kindle_royalty(&self) -> f64{
        if self.kindle > TIER1.start{
            self.kindle * 0.35
        } else {
            self.kindle * 0.7
        }
    }

    fn check_best_kindle(&self, new_price: f64) -> f64{
        let curr_royalty = self.kindle_royalty();
        let optimal = TIER1.start * 0.7;
        let new_royalty = if new_price <= TIER1.start{
            new_price * 0.7
        } else {
            new_price * 0.35
        };

        match curr_royalty.total_cmp(&new_royalty){
            Ordering::Less => {
                max(new_royalty, optimal)
            }
            Ordering::Equal => {
                max(curr_royalty, optimal)
            }
            Ordering::Greater => {
                max(curr_royalty, optimal)
            }
        }
    }

    pub fn adjust(&mut self) -> Result<()>{

        Ok(())
    }
}

impl Default for BookPrices {
    fn default() -> Self {
        Self {
            ebook: TIER1.start,
            kindle: TIER2.start,
            paperback: TIER3.start,
            hardcover: TIER5.start,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ProductType {
    // Free material
    Article,
    HowToGuide,
    CheatSheet,
    // Paid material
    Publication,
    Udemy,
}

impl ProductType{
    fn path() -> PathBuf{
        PathBuf::from("products")
    }
    pub fn prod_path(&self) -> PathBuf{
        match self{
            ProductType::Article => Self::path().join("articles.json"),
            ProductType::HowToGuide => Self::path().join("howtoguides.json"),
            ProductType::CheatSheet => Self::path().join("cheatsheets.json"),
            ProductType::Publication => Self::path().join("publications.json")
            ProductType::Udemy => {}
        }
    }
}

impl Product {
    fn new_publication(name: String) -> Self {
        Self {
            name,
            prod_ty: ProductType::Publication,
            book_price: Some(BookPrices::default()),
            udemy_price: None,
        }
    }
    fn new_udemy(name: String) -> Self {
        Self{
            name,
            prod_ty: ProductType::Udemy,
            book_price: None,
            udemy_price: Some(UDEMY_DEF)
        }
    }

    fn new_material(name: String, prod_ty: ProductType) -> Self{
        Self{name, prod_ty, book_price: None, udemy_price: None}
    }

    pub fn new(name: String, prod_ty: ProductType) -> Self{
        match prod_ty{
            ProductType::Publication => Self::new_publication(name),
            ProductType::Udemy => Self::new_udemy(name),
            _ => Self::new_material(name, prod_ty)
        }
    }
}
