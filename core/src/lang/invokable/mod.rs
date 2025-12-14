pub mod operator;
pub mod unary;

use crate::lang::SuperType;

pub trait Invokable {
    fn invoke_1(&self, _arg1: SuperType) -> Result<SuperType, crate::Error> {
        Err(crate::Error::Rank)
    }

    fn invoke_2(&self, _arg1: SuperType, _arg2: SuperType) -> Result<SuperType, crate::Error> {
        Err(crate::Error::Rank)
    }

    fn invoke_3(
        &self,
        _arg1: SuperType,
        _arg2: SuperType,
        _arg3: SuperType,
    ) -> Result<SuperType, crate::Error> {
        Err(crate::Error::Rank)
    }

    fn invoke_4(
        &self,
        _arg1: SuperType,
        _arg2: SuperType,
        _arg3: SuperType,
        _arg4: SuperType,
    ) -> Result<SuperType, crate::Error> {
        Err(crate::Error::Rank)
    }

    fn invoke_5(
        &self,
        _arg1: SuperType,
        _arg2: SuperType,
        _arg3: SuperType,
        _arg4: SuperType,
        _arg5: SuperType,
    ) -> Result<SuperType, crate::Error> {
        Err(crate::Error::Rank)
    }

    fn invoke_6(
        &self,
        _arg1: SuperType,
        _arg2: SuperType,
        _arg3: SuperType,
        _arg4: SuperType,
        _arg5: SuperType,
        _arg6: SuperType,
    ) -> Result<SuperType, crate::Error> {
        Err(crate::Error::Rank)
    }

    fn invoke_7(
        &self,
        _arg1: SuperType,
        _arg2: SuperType,
        _arg3: SuperType,
        _arg4: SuperType,
        _arg5: SuperType,
        _arg6: SuperType,
        _arg7: SuperType,
    ) -> Result<SuperType, crate::Error> {
        Err(crate::Error::Rank)
    }

    fn invoke_8(
        &self,
        _arg1: SuperType,
        _arg2: SuperType,
        _arg3: SuperType,
        _arg4: SuperType,
        _arg5: SuperType,
        _arg6: SuperType,
        _arg7: SuperType,
        _arg8: SuperType,
    ) -> Result<SuperType, crate::Error> {
        Err(crate::Error::Rank)
    }
}
