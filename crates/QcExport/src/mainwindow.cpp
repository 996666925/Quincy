#include "mainwindow.h"
#include "qlogging.h"
#include "qtypeinfo.h"
#include "src/lib.rs.h"
#include "ui_mainwindow.h"
#include <QDragEnterEvent>
#include <QMimeData>
#include <qDebug>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent), ui(new Ui::MainWindow) {
  ui->setupUi(this);
  this->setAcceptDrops(true);
}

MainWindow::~MainWindow() { delete ui; }

void MainWindow::dragEnterEvent(QDragEnterEvent *e) {

  e->acceptProposedAction();
}

void MainWindow::dropEvent(QDropEvent *e) {
  // 获取文件路径 (QString)
  QList<QUrl> urls = e->mimeData()->urls();
  if (urls.isEmpty())
    return;
  QString qStr = urls.first().toLocalFile();

  // 转为char*
  QByteArray qByteArrary = qStr.toLatin1();
  char *filePath = qByteArrary.data();

  QcExport::create_mesh(rust::Str(filePath));


}
