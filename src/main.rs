use rust_bert::pipelines::sentiment::SentimentModel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sentiment_classifier = SentimentModel::new(Default::default())?;
   
    let input = [ 
        "Nvidia Stock Is Falling After Earnings. It's Just One More Big Tech Disappointment.
        Nvidia recently said it was slowing down the pace of its hiring. DREAMSTIME
        Nvidia stock was falling after the chip maker provided a softer-than-expected outlook for its July quarter. The company cited the impact of both reduced business in Russia and Covid-related manufacturing shutdowns in China.
        In March, Nvidia had announced the halt of all product sales in Russia. Meanwhile, the issues involving Covid-related manufacturing shutdowns in China are consistent with recent reports from other companies, including both Cisco Systems (CSCO) and Apple (AAPL).
        Nvidia CFO Colette Kress said in an interview that ending shipments to Russia affected end-market sales to gaming, high-end graphics, and data center customers in the country. She noted that the China issues affects both supply and demand, with the economy in some areas effectively shut down.
        Nvidia's results for the fiscal first quarter ended April 30 were actually ahead of expectations. Nvidia posted revenue of $8.29 billion, up 46% from a year ago, and well ahead of the company's forecast and Wall Street's estimate, both $8.1 billion. On an adjusted basis, Nvidia earned $1.36 a share, ahead of the analyst consensus of $1.29 a share.
        But for the July quarter, Nvidia sees revenue of $8.1 billion, give or take 2%, after a $500 million reduction relating to Russia and the Covid lockdowns in China. Wall Street consensus had called for $8.5 billion. Kress confirms that guidance would have been above Wall Street estimates without those factors. The company sees July quarter gross margin of 65.1% on a GAAP basis, and 67.1% on a non-GAAP basis.
        In the April quarter, gross margin was 67.1% on a non-GAAP basis, and 65.5% under generally accepted accounting principles.
        Nvidia said data center-related revenue was $3.75 billion, up 83% from a year ago and 15% higher sequentially, and now represent the company's single-largest end market. Kress says revenue from the segment was a record, with strong demand from leading cloud service providers like Amazon, Microsoft and Alphabet ; she expect even higher revenue in the July quarter.
        Gaming segment revenue was $3.62 billion, up 31%, and 6% sequentially. Revenue in professional visualization was $622 million, up 67% from a year ago, but off 3% from the previous quarter.
        Automotive and robotics segment revenue was $138 million, down 10% from a year ago, but up 10% from the fiscal fourth quarter. The company also said that cryptocurrency mining processor revenue was nominal in the quarter, compared with $155 million in the year ago period. Kress said that while the volume of graphics processors used for crypto mining isn't clear, it is possible that sales have slipped amid the recent decline in cryptocurrency prices.
        Nvidia added that it saw lower demand in the quarter for processors used in entry-level notebook computers.
        Nvidia CEO Jensen Huang said in statement that the company posted record results in the data center and gaming segments against the backdrop of a challenging macro environment.
        Nvidia shares are down 4.3% in premarket trading Thruisday. Heading into tonight's results, the stock was down 42% this year.
        Nvidia also said its board approved an expansion of the company's stock repurchase program to $15 billion. In the quarter, the company repurchased $2 billion of stock, for a total return to shareholders including dividends of $2.1 billion."
    ];

    let output = sentiment_classifier.predict(&input);

    println!("{:?}", output);

    return Ok(());
}
