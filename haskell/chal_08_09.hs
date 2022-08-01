import Data.Bifunctor (Bifunctor (bimap, first, second))
import Data.Functor.Const (Const (Const))
import Data.Functor.Identity (Identity (Identity))

-- 1.

data Pair a b = Pair a b

instance Bifunctor Pair where
    bimap f g (Pair a b) = Pair (f a) (g b)
    first f (Pair a b) = Pair (f a) b
    second f (Pair a b) = Pair a (f b)

-- 2.

type Maybe' a = Either  (Const () a) (Identity a)

isom :: Maybe a -> Maybe' a
isom Nothing = Left $ Const ()
isom (Just a) = Right $ Identity a

isom' :: Maybe' a -> Maybe a
isom' (Left (Const ())) = Nothing
isom' (Right (Identity a)) = Just a
{-
isom . isom' $ Nothing 
    = isom $ Left (Const ())
    = Nothing 
isom . isom' $ Just a 
    = isom $ Right (Identity a)
    = Just a

=> isom . isom' = id

isom' . isom $ Left (Const ())
    = isom' $ Nothing
    = Left (Const ())
isom' . isom $ Right (Identity a)
    = isom' $ Just a
    = Right (Identity a)

=> isom' . isom = id
-}


-- 3.

data PreList a b = Nil | Cons a b
instance Bifunctor PreList where
  bimap f g Nil = Nil
  bimap f g (Cons a b) = Cons (f a) (g b)
  first f Nil = Nil
  first f (Cons a b) = Cons (f a) b
  second g Nil = Nil
  second g (Cons a b) = Cons a (g b)

-- 4. 

newtype K2 c a b = K2 c
instance Bifunctor (K2 c) where 
    bimap _ _ (K2 c) = K2 c

newtype Fst a b = Fst a
instance Bifunctor Fst where
    bimap f _ (Fst a) = Fst (f a)

newtype Snd a b = Snd b
instance Bifunctor Snd where
    bimap _ g (Snd b) = Snd $ g b