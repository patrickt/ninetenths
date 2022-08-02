let System = ./bio_system.dhall

in  let BodyPart =
          { Type = { name : Text, purpose : System }
          , default.name = None Text
          , default.purpose = System.Tissue
          }

    in  BodyPart
