import { useCallback, useEffect, useState } from 'react';
import { Route, Routes } from 'react-router-dom';
import { ACCESS_TOKEN } from './access_token';
import FoodList from './components/FoodList';

function App() {
  const [foods, setFoods] = useState([]);

  let fetchFoods = useCallback(async function () {
    let response: Response = await fetch(`/api/food?access_token=${ACCESS_TOKEN}`,
      {
        method: "GET"
      });

    if (response.ok) {
      let data = await response.json();
      setFoods(data.foods);
    } else {
    }

  }, []);

  useEffect(() => {
    fetchFoods();
  }, [fetchFoods]);

  return (
    <Routes>
      <Route path="/">
        <Route index element={<FoodList foods={foods} />} />
      </Route>
    </Routes>
  );
}

export default App;
