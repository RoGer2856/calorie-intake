import { ReactElement, useCallback, useEffect, useState } from 'react';
import AdminApp from './components/AdminApp';
import Loading from './components/Loading';
import RegularUserApp from './components/RegularUserApp';
import useApi from './hooks/use-api';
import { IUserInfo, Role } from './model/UserInfo';

function App() {
  const api = useApi();

  const [userInfo, setUserInfo] = useState<IUserInfo | null>(null);

  let fetchUserInfo = useCallback(async function () {
    let response = await api.getUserInfo();
    if (response !== null) {
      setUserInfo(response as IUserInfo);
    }
  }, []);

  useEffect(() => {
    fetchUserInfo();
  }, [fetchUserInfo])

  return (
    <>
      {userInfo !== null
        ?
        <>
          {userInfo!.role === Role.RegularUser
            ?
            <RegularUserApp />
            :
            <AdminApp />
          }
        </>
        :
        <Loading />
      }
    </>
  );
}

export default App;
