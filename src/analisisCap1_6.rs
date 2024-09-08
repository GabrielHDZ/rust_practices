struct Pelota{
    diametro:u32,
    material:String,
}

impl Pelota{
    fn new(diametro:u32,material:String)->Pelota{
        Pelota{
            diametro,
            material,
        }
    }
    fn get(&self){
        println!("Pelota de futbol hecha de material :{:?} y diametro {:?}",self.material,self.diametro);
    }
    fn set_diametro(&mut self,new_diam:u32){
        self.diametro=new_diam;
    }
}

enum TypePelota{
    Futbol,
    Americano,
    Beisbol,
    Basquetbol,
}

pub fn run(){
    //Creamos instancia de Pelota
    let mut p_futbol:Pelota=Pelota::new(30,String::from("Caucho"));
    //imprimimos los datos ingresados
    p_futbol.get();
    p_futbol.set_diametro(20);
    p_futbol.get();
    match TypePelota {
        TypePelota::Futbol=>println!("Es una pelota de futbol"),
        TypePelota::Americano=>println!("Es una pelota de futbol americano"),
        TypePelota::Basquetbol=>println!("pelota de basquetbol"),
        TypePelota::Beisbol=>println!("pelota de basquetbol"),
    }
}

