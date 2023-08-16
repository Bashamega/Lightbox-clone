use warp::{Filter, reply::html};

#[tokio::main]
async fn main() {
    // Define a warp filter for the home page
    let home = warp::path::end()
        .map(|| {
            let content= format!(r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name='viewport' content="width=device-width, initial-scale=1.0">
                <title>Your Rust Website</title>
                <link rel="stylesheet" href="https://unpkg.com/tailwindcss@2.2.16/dist/tailwind.min.css">
            </head>
            
            
            <body class="bg-gray-900 text-white">
                <nav class="flex justify-between items-center p-4">
                    <h1>Lightbox</h1>
                    <div id='center' class="flex items-center space-x-4 rounded-full bg-gray-800 p-2">
                        <a href='#'>Discover coins</a>
                        <a href='#'>Trading</a>
                        <a href='#'>AL FAI Score</a>
                        <a href='#'>Crypto Calendar</a>
                        <a href='#'>About Dashboard</a>
                    </div>
                    <div class="space-x-4">
                        <a href='#'>Sign in</a>
                        <a href='#' class="bg-gray-800 p-2 rounded">Sign up</a>
                    </div>
                </nav>
                <section class="text-center relative">
                    <div class="squares bg-gray-800" style="--nr: 20; /* number of rows */
                    --nc: 20; /* number of columns */
                    --b: 2px; /* border length */
                
                    width: 600px;
                    height: 500px;
                    margin: 10px auto;
                
                    --m: conic-gradient(from 90deg at var(--b) var(--b), red 90deg, #0000 0) calc(-1 * var(--b)) calc(-1 * var(--b)) / calc(100% / var(--nc)) calc(100% / var(--nr));
                    -webkit-mask: var(--m);
                    mask: var(--m);
                
                    background: rgb(65, 64, 64);"></div>
                    <section id="content" class="absolute top-0 left-0 p-20 flex flex-col items-center text-center w-full h-full box-border">
                        <br>
                        <a class="bg-gradient-to-b from-gray-600 to-gray-800 p-1 rounded" href='#'>Lightbox raises $3M seed round -></a>
                        <br>
                        <h1 class='text-4xl font-bold mb-4'>Snipe Smarter, Not Harder</h1>
                        <br>
                        <p class='text-gray-800'>
                            Cut through the noise of coin sniping with Lightbox, Harness real-time data, Ai-<br>generated insights, and powerful tools to supercharge your trading strategy
                        </p>
                        <br>
                        <button class='rounded p-1 bg-blue-500'>Get started for free</button>
                    </section>
                </section>
            </body>
            </html>
            
            "#);
            html(content)
        });

    

    // Combine all filters
    let routes = home.or(style);

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

