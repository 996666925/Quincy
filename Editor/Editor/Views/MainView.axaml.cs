using Avalonia.Controls;

using NP.Ava.UniDock;
using NP.Ava.Visuals.Behaviors;
using System.Collections.ObjectModel;
using Avalonia.Interactivity;

namespace Editor.Views;

public partial class MainView : UserControl
{


    public MainView()
    {


        InitializeComponent();
     

    }

 
    protected override void OnUnloaded(RoutedEventArgs e)
    {
        base.OnUnloaded(e);
        
 
    }
}
