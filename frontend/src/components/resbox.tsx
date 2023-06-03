import React, { useEffect, useState } from 'react';
import axios from 'axios';

interface ResourceData {
  name: string;
  url: string;
  response: number;
}

const ResourceBox: React.FC = () => {
  const [resource, setResource] = useState<ResourceData | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const fetchData = async () => {
    try {
      const response = await axios.get('/res');
      setResource(response.data);
    } catch (error) {
      console.log('Error:', error);
    }
  };

  useEffect(() => {
    

    fetchData();
  }, []);

  const getColor = (response: number) => {
    return response === 200 ? 'green' : 'red';
  };

  const handleButtonClick = async () => {
    try {
      setIsLoading(true);

      await axios.post('/get', {
        url: resource?.url,
        name: resource?.name,
      }).then(r=>{
        fetchData()
      });

      setIsLoading(false);
    } catch (error) {
      console.log('Error:', error);
      setIsLoading(false);
    }
  };

  return (
    <div>
      {resource && (
        <button
          onClick={handleButtonClick}
          style={{
            backgroundColor: isLoading ? 'orange' : getColor(resource.response),
            padding: '10px',
            color: 'white',
            fontWeight: 'bold',
          }}
        >
          {isLoading ? 'Loading...' : resource.name}
        </button>
      )}
    </div>
  );
};

export default ResourceBox;
