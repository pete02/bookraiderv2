import React, { useState } from 'react';
import Search from './components/search';
import ResourceBox from './components/resbox';
import AddSite from './components/add';
import './App.css';

const App: React.FC = () => {
  const [showAddComponent, setShowAddComponent] = useState(false);

  const handleToggleComponent = () => {
    setShowAddComponent((prevState) => !prevState);
  };

  return (
    <div className="container">
      {!showAddComponent ? (
        <>
          <ResourceBox />
          <br />
          <Search />
          <br />
          <button onClick={handleToggleComponent}>Switch to Add Component</button>
        </>
      ) : (
        <AddSite />
      )}
    </div>
  );
};

export default App;
