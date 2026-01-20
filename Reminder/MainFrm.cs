using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Windows.Forms;

namespace Reminder
{
    public partial class MainFrm : Form
    {
        WorkFrm wrkFrm;
        public MainFrm()
        {
            InitializeComponent();
        }

        private void MainFrm_Load(object sender, EventArgs e)
        {
            try
            {
                using (var stream = System.Reflection.Assembly.GetExecutingAssembly().GetManifestResourceStream("Reminder.ICO2.ico"))
                {
                    if (stream != null)
                    {
                        notifyIcon1.Icon = new Icon(stream);
                    }
                }
            }
            catch
            {
                notifyIcon1.Icon = System.Drawing.SystemIcons.Application;
            }
            notifyIcon1.Visible = true;
        }
       

        private void Btn_start_Click(object sender, EventArgs e)
        {
            bool input_flag;

            if (this.ckBoxInput.Checked)
            {
                input_flag = true;
            }
            else {
                input_flag = false;
            }

            int wrkTime = (int)this.numWrkTime.Value;
            int rstTime = (int)this.numRstTime.Value;
            
            StopOldTimers();
            
            wrkFrm = new WorkFrm(wrkTime,rstTime,input_flag);
            wrkFrm.Show();
            this.Visible = false;

        }

        private void StopOldTimers()
        {
            if (wrkFrm != null && !wrkFrm.IsDisposed)
            {
                wrkFrm.StopTimer();
                wrkFrm.Close();
            }

            List<Form> formsToClose = new List<Form>();
            foreach (Form f in Application.OpenForms)
            {
                if (f is WorkFrm || f is RestFrm)
                {
                    formsToClose.Add(f);
                }
            }
            foreach (Form f in formsToClose)
            {
                if (f is WorkFrm)
                {
                    ((WorkFrm)f).StopTimer();
                }
                if (f is RestFrm)
                {
                    ((RestFrm)f).IsAborted = true;
                    ((RestFrm)f).StopTimer();
                }
                f.Close();
            }
        }

        private void 主窗体ToolStripMenuItem_Click(object sender, EventArgs e)
        {            
            this.Visible = true;
            this.WindowState = FormWindowState.Normal;
            StopOldTimers();
        }

        private void MainFrm_FormClosing(object sender, FormClosingEventArgs e)
        {            
            e.Cancel = true;
            this.WindowState = FormWindowState.Minimized;
            this.Visible = false;
            this.ShowInTaskbar = false;
            notifyIcon1.Visible = true;
        }

        private void 退出ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            notifyIcon1.Visible = false;
            System.Environment.Exit(0);
        }

        private void 关于ToolStripMenuItem_Click(object sender, EventArgs e)
        {
            AboutBox aboutBox = new AboutBox();
            aboutBox.ShowDialog();
        }

        private void notifyIcon1_DoubleClick(object sender, EventArgs e)
        {
            this.Visible = true;
            this.WindowState = FormWindowState.Normal;
            this.ShowInTaskbar = true;
            this.BringToFront();
            this.Activate();
        }
    }
}
