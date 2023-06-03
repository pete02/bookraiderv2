import React, { useState } from 'react';
import axios from 'axios';

const AddSite: React.FC = () => {
    const [site, setSite] = useState({
        head: '',
        url: '',
        search: '',
        filters: [] as string[], // Set the type of filters as an empty array of strings
        container: '',
        classname: '',
        page: '',
        title: '',
      });

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;

    if (name === 'filters') {
      const filters = value.split(',').map((filter) => filter.trim());
      setSite((prevSite) => ({ ...prevSite, filters }));
    } else {
      setSite((prevSite) => ({ ...prevSite, [name]: value }));
    }
  };

  const handleAddSite = async () => {
    try {
      await axios.post('/add', site);
      console.log('Site added successfully!');
    } catch (error) {
      console.log('Error:', error);
    }
  };

  return (
    <div>
      <h2>Add Site</h2>
      <form>
        <label>
          Head:
          <input type="text" name="head" value={site.head} onChange={handleChange} />
        </label>
        <br />
        <label>
          URL:
          <input type="text" name="url" value={site.url} onChange={handleChange} />
        </label>
        <br />
        <label>
          Search:
          <input type="text" name="search" value={site.search} onChange={handleChange} />
        </label>
        <br />
        <label>
          Filters:
          <input type="text" name="filters" value={site.filters.join(', ')} onChange={handleChange} />
          <small>Separate each filter with a comma (,)</small>
        </label>
        <br />
        <label>
          Container:
          <input type="text" name="container" value={site.container} onChange={handleChange} />
        </label>
        <br />
        <label>
          Classname:
          <input type="text" name="classname" value={site.classname} onChange={handleChange} />
        </label>
        <br />
        <label>
          Page:
          <input type="text" name="page" value={site.page} onChange={handleChange} />
        </label>
        <br />
        <label>
          Title:
          <input type="text" name="title" value={site.title} onChange={handleChange} />
        </label>
        <br />
        <button type="button" onClick={handleAddSite}>Add Site</button>
      </form>
    </div>
  );
};

export default AddSite;
