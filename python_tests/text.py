from imports import *

def test(btn_id):
    update_widget(wid=id, param=IpgTextParam.AlignBottomCenter, value=None)
    update_widget(wid=id, param=IpgTextParam.AlignBottomCenter, value=True)
    update_widget(wid=id, param=IpgTextParam.AlignBottomLeft, value=None)
    update_widget(wid=id, param=IpgTextParam.AlignBottomLeft, value=True)
    update_widget(wid=id, param=IpgTextParam.AlignBottomRight, value=None)
    update_widget(wid=id, param=IpgTextParam.AlignBottomRight, value=True)
    update_widget(wid=id, param=IpgTextParam.Content, value="Some")
    update_widget(wid=id, param=IpgTextParam.Height, value=None)
    update_widget(wid=id, param=IpgTextParam.Height, value=100.0)
    update_widget(wid=id, param=IpgTextParam.LineHeight, value=None)
    update_widget(wid=id, param=IpgTextParam.LineHeight, value=1.5)
    update_widget(wid=id, param=IpgTextParam.Show, value=False)
    update_widget(wid=id, param=IpgTextParam.Size, value=None)
    update_widget(wid=id, param=IpgTextParam.Size, value=20.0)
    update_widget(wid=id, param=IpgTextParam.TextColor, value=None)
    update_widget(wid=id, param=IpgTextParam.TextColor, value=IpgColor.BLACK)
    update_widget(wid=id, param=IpgTextParam.TextRgba, value=None)
    update_widget(wid=id, param=IpgTextParam.TextRgba, value=[0.0, 4])
    update_widget(wid=id, param=IpgTextParam.TextShaping, value=None)
    update_widget(wid=id, param=IpgTextParam.TextShaping, value=TextShaping.Auto)
    update_widget(wid=id, param=IpgTextParam.TextWrapping, value=None)
    update_widget(wid=id, param=IpgTextParam.TextWrapping, value=TextWrapping.Word)
    update_widget(wid=id, param=IpgTextParam.Width, value=None)
    update_widget(wid=id, param=IpgTextParam.Width, value=100.0)
    update_widget(wid=id, param=IpgTextParam.WidthFill, value=None)
    update_widget(wid=id, param=IpgTextParam.WidthFill, value=True)
    
    

# Add a window
with Window(id="main"):

    with Column():
        
        id = add_text(content="Some Text")
        add_button(label="Start", on_press=test)
        

start_session()