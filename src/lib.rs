/*
MoKa Reads Core Application
Developed by Mustafif Khan

This application is built to serve as a helper for various MoKa Reads analytics and suggestions
when organizing and creating new products or services.

All code is under the GPLv2 License (LICENSE.md).
 */

/// CPI represents the Consumer Price Index from the Bank of Canada
#[allow(non_snake_case)]
pub mod cpi;
/// Represents the suggested pricing adjustment according to the annul inflation rate
///
/// All prices are under $CAD
pub mod prices;
/// Represents the creation and suggested price markings of a product
pub mod products;
