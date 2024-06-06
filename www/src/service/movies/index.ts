import axios from "axios";

export class MovieService {
  static async get_random_movies() {
    const res = await axios.get(`http://localhost:8000/api/v1/movies`, {
      headers: {
        "Content-Type": "application/json",
        "x-auth-token": import.meta.env.VITE_TOKEN,
      },
    });

    return res.data;
  }

  static async get_download_links(slug: string) {
    const res = await axios.get(`http://localhost:8000/api/v1/download/${slug}`, {
      headers: {
        "Content-Type": "application/json",
        "x-auth-token": import.meta.env.VITE_TOKEN,
      },
    });

    return res.data;
  }
}
