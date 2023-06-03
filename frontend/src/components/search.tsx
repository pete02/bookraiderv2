import React, { useState } from 'react';
import axios from 'axios';
import '../App.css';

interface SiteData {
  Site: string;
  res: Resource[];
}

interface Resource {
  name: string;
  url: string;
}

const Search: React.FC = () => {
  const [results, setResults] = useState<SiteData[]>([]);
  const [collapsedSites, setCollapsedSites] = useState<boolean[]>([]);
  const [name, setName] = useState('');
  const [loading, setLoading] = useState(false);
  const [responseMessage, setResponseMessage] = useState('');
  const [writer,SetWriter]=useState('')

  const handleNameChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setName(event.target.value);
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setLoading(true);
    setResponseMessage('');
    try {
      const response = await axios.get(`/search/${name.replace(" ", "+")}`);
      setResults(response.data);
      setCollapsedSites(new Array(response.data.length).fill(false));
    } catch (error) {
      console.log('Error:', error);
    }
    setLoading(false);
  };

  const handleWriterChange=(event: React.ChangeEvent<HTMLInputElement>)=>{
    SetWriter(event.target.value)
  } 

  const handleCollapse = (index: number) => {
    setCollapsedSites((prevState) => {
      const updatedCollapsedSites = [...prevState];
      updatedCollapsedSites[index] = !prevState[index];
      return updatedCollapsedSites;
    });
  };

  const handleGet = async (resource: Resource) => {
    setLoading(true);
    setResponseMessage('');
    try {
      const response = await axios.post("/get", {
        url: resource.url,
        name: name,
        writer: writer
      });
      setResponseMessage(`Response: ${response.status}`);
      console.log(response);
    } catch (error) {
      console.log('Error:', error);
    }
    setLoading(false);
  };

  return (
    <div className="container">
      <h1>Search Functionality</h1>
      <form onSubmit={handleSubmit}>
        <label>
          Name:
          <input type="text" value={name} onChange={handleNameChange} />
        </label>
        <br/>
        <label>
          Writer:
          <input type="text" value={writer} onChange={handleWriterChange} />
        </label>
        <button type="submit">Get</button>
      </form>
      <div>
        {loading && <p>Loading...</p>}
        {responseMessage && <p>{responseMessage}</p>}
        {results.map((site, index) => (
          <div key={site.Site}>
            <p onClick={() => handleCollapse(index)} className="site-name">
              {site.Site}
            </p>
            {collapsedSites[index] && (
              <ul>
                {site.res.map((resource, resourceIndex) => (
                  <p key={resource.url}>
                    <a href={resource.url} className="resource-link">
                      {resource.name}
                    </a>
                    <button disabled={loading} onClick={() => handleGet(resource)}>
                      Get
                    </button>
                  </p>
                ))}
              </ul>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

export default Search;
