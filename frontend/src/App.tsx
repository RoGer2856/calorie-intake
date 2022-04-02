import { Route, Routes } from 'react-router-dom';
import MyFoods from './components/MyFoods';

function App() {
  return (
    <Routes>
      <Route path="/">
        <Route index element={<MyFoods />} />
      </Route>
    </Routes>
  );
}

export default App;
