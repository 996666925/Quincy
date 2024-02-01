using NP.Ava.UniDock;
using NP.Ava.UniDock.Factories;
using NP.Ava.UniDockService;
using NP.DependencyInjection.Interfaces;


namespace Editor
{
    public static class MyContainer
    {


        public static DockManager TheDockManager { get; } = new DockManager();

        static MyContainer()
        {
  
            TheDockManager.IsInEditableState = true;
        }
    }
}
