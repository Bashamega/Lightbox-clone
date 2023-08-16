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
            
            
            <body class="bg-gray-900 text-white w-screen">
                <!-- Inside the <body> section -->
                    <div id="squaresGrid" class="" style="display: grid;
                        grid-template-columns: repeat(auto-fill, minmax(10px, 1fr));
                        gap: 20px;
                        max-width: 100%;
                        margin: 0 auto;
                        padding: 2px;">
                    </div>
                    <script>
                        // Calculate the number of rows based on the total height of the content
// Define the variables for boxSize and spacing
const boxSize = 50;
const spacing = 20;

// Calculate the number of columns based on the width of the viewport
const numCols = Math.floor((window.innerWidth - 4) / (boxSize + spacing));

// Calculate the number of rows based on the total height of the content
const totalContentHeight = numCols * (boxSize + spacing);
const windowHeight = window.innerHeight;
const numVisibleRows = Math.ceil(windowHeight / (boxSize + spacing));
const numTotalRows = Math.ceil(totalContentHeight / (boxSize + spacing));

const gridContainer = document.getElementById('squaresGrid');
gridContainer.style.gridTemplateColumns = `repeat(${numCols}, ${boxSize}px)`;
gridContainer.style.gridTemplateRows = `repeat(${numTotalRows}, ${boxSize}px)`;
gridContainer.style.height = `${totalContentHeight}px`; // Set the height
const options = ['lime', 'none'];

for (let row = 0; row < numVisibleRows; row++) {
    for (let col = 0; col < numCols; col++) {
        const box = document.createElement('div');
        const className = options[Math.floor(Math.random() * 2)];
        if (className == "lime") {
            box.style.backgroundColor = 'rgb(132 204 22)';
        } else {
            box.classList.add(className);
        }
        box.classList.add('square', 'rounded-full');
        box.style.width = `${boxSize}px`;
        box.style.height = `${boxSize}px`;
        box.style.marginRight = `${spacing}px`;
        box.style.marginBottom = `${spacing}px`;
        gridContainer.appendChild(box);
    }
}

                    </script>

                    
                    

                <section class="absolute top-0 left-0 w-full">
                    <nav class="flex justify-between items-center p-4 bg-gray-900">
                        <h1 class="text-3xl font-bold mb-4">Lightbox</h1>
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
                        
                        
                        <section id="content" class=" flex flex-col items-center text-center w-full h-full box-border ">
                            <img src="https://thumbs.gfycat.com/HeftyFaintAngelfish-max-1mb.gif" class="text-center">
                            <br>
                            <a class="bg-gradient-to-b from-gray-600 to-gray-800 p-1 rounded" href='#'>Lightbox raises $3M seed round -></a>
                            <br>
                            <h1 class='text-4xl font-bold mb-4'>Snipe Smarter, Not Harder</h1>
                            <br>
                            <p class='text-gray-500'>
                                Cut through the noise of coin sniping with Lightbox, Harness real-time data, Ai-<br>generated insights, and powerful tools to supercharge your trading strategy
                            </p>
                            <br>
                            <button class='rounded p-1 bg-blue-500'>Get started for free</button>
                            <br><br>
                            <img class="w-52" src="https://i1.wp.com/gelatologia.com/wp-content/uploads/2020/07/placeholder.png?ssl=1" alt="image holder">
                        </section>
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

