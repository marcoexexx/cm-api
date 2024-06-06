import { useEffect, useState } from "react";
import { MovieService } from "./service/movies";

function App() {
  const [count, setCount] = useState<number>(0);
  const [movies, setMovies] = useState<Array<Record<string, string>> | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);
  const [loading, setLoading] = useState<boolean>(true);

  async function get_download_links(slug: string) {
    console.log("loading...");
    let links = await MovieService.get_download_links(slug);
    console.clear();
    console.log(links);
  }

  useEffect(() => {
    MovieService.get_random_movies()
      .then(movies => {
        setMovies(movies.result);
        setCount(movies.count);
      })
      .catch(err => {
        console.log("ERROR", err);
        setError(err?.response?.data?.message);
      })
      .finally(() => {
        setLoading(false);
      });
  }, []);

  if (loading) return <h3>Loading...</h3>;
  if (error) return <pre>{error}</pre>;

  return (
    <div>
      <h3>Total: {count}</h3>

      {movies?.map((movie, idx) => (
        <div key={idx}>
          <img src={movie.poster} />
          <h4>{movie.title}</h4>
          <button
            onClick={async () => await get_download_links(movie.slug)}
          >
            Download
          </button>
        </div>
      ))}
    </div>
  );
}

export default App;
