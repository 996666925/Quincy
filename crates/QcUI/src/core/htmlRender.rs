use crate::component::HtmlRender;



pub trait HtmlRenderTrait{
    fn draw(&self);
}

impl HtmlRenderTrait for HtmlRender{
    fn draw(&self){
        for (_, comp) in self.iter() {
            
        }
    }

  
}