import './App.scss'
import { useEffect, useState } from 'react'
import { Header } from 'components/header/Header'
import useNav from 'routes';

const App = () => {
  const { element: routes, menu } = useNav();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const init = async () => {

      setLoading(false)
    }
    init();
  }, [])

  return (
    <div className='App'>
      <Header menu={menu} />
      <div className='AppContent'>{routes}</div>
    </div>
  )
}

export default App
