import './App.scss'
import { useEffect, useState } from 'react'
import { Header } from 'components/header/Header'
import useNav from 'routes';

const App = () => {
  const { element: routes } = useNav();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const init = async () => {
      
      setLoading(false)
    }
    init();
  }, [])
  
  return (
    <div className="App">
      <Header />
      {routes}
    </div>
  )
}

export default App
