let $ = (...args) => window.document.querySelector(args);

const BASE_URL = Object.freeze("https://cm-api.shuttleapp.rs/");
const ROOT_DOCUMENT = Object.freeze(window.document.querySelector("#root"));

async function main() {
  let movies = await fetch(BASE_URL);

  /** @type {{ cm_link: string; image: string; slug: string; title: string}[]} */
  let data = await movies.json();

  if (!ROOT_DOCUMENT) return;

  data.forEach((movie, index) => {
    let container_div = window.document.createElement("div");

    let title_h3 = window.document.createElement("h3");
    title_h3.textContent = `[ ${index + 1} ] ${movie.title}`;
    container_div.appendChild(title_h3);

    let image_img = window.document.createElement("img");
    image_img.src = movie.image;
    container_div.appendChild(image_img);

    ROOT_DOCUMENT.appendChild(container_div);
  });
}

main().catch(console.error);
